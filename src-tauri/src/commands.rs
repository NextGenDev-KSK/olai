//! Thin Tauri command handlers. Each validates input, calls one service or the
//! pipeline, and returns typed results. Errors serialize as `UserFacingError`.
//! No business logic lives here.

use crate::error::{AppError, AppResult};
use crate::fixtures;
use crate::models::{
    Analysis, DemoScenarioKind, DemoSession, Document, IngestFile, IngestKind, Language, Page,
    QaAnswer, ReplyDraft, SessionId, SourceKind,
};
use crate::providers::anthropic::AnthropicProvider;
use crate::providers::LlmProvider;
use crate::services::{extraction, injection, pipeline, segmentation};
use crate::state::AppState;
use crate::{fixtures::Scenario, secrets};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Local;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tauri::State;

static SESSION_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate an opaque, unique in-memory session id.
fn new_session_id() -> String {
    format!("session-{}", SESSION_COUNTER.fetch_add(1, Ordering::SeqCst))
}

/// Strip any directory components from an untrusted file name.
fn base_name(name: &str) -> String {
    name.rsplit(['/', '\\']).next().unwrap_or(name).to_string()
}

/// Build the real Anthropic provider from the stored key.
fn real_provider() -> AppResult<Arc<dyn LlmProvider>> {
    let key = secrets::get_key()?.ok_or(AppError::MissingApiKey)?;
    Ok(Arc::new(AnthropicProvider::new(key)?))
}

/// Today's local date, used for deadline arithmetic.
fn today() -> chrono::NaiveDate {
    Local::now().date_naive()
}

// ---- API key ----------------------------------------------------------------

/// Whether an API key is stored.
#[tauri::command]
async fn has_api_key() -> AppResult<bool> {
    secrets::has_key()
}

/// Store the API key in the Windows Credential Manager.
#[tauri::command]
async fn set_api_key(key: String) -> AppResult<()> {
    let trimmed = key.trim();
    if trimmed.is_empty() || !trimmed.starts_with("sk-") {
        return Err(AppError::Validation("that does not look like an API key".into()));
    }
    secrets::set_key(trimmed)
}

/// Delete the stored API key.
#[tauri::command]
async fn delete_api_key() -> AppResult<()> {
    secrets::delete_key()
}

// ---- Session lifecycle ------------------------------------------------------

/// Start a new session in the given language.
#[tauri::command]
async fn start_session(state: State<'_, AppState>, language: Language) -> AppResult<SessionId> {
    let id = new_session_id();
    state.create_session(&id, language, false)?;
    Ok(SessionId(id))
}

/// Change a session's language.
#[tauri::command]
async fn set_language(
    state: State<'_, AppState>,
    session: SessionId,
    language: Language,
) -> AppResult<()> {
    state.set_language(&session.0, language)
}

/// Set (or clear) the received-date anchor.
#[tauri::command]
async fn set_received_date(
    state: State<'_, AppState>,
    session: SessionId,
    date: Option<String>,
) -> AppResult<()> {
    state.set_anchor(&session.0, date)
}

// ---- Ingestion --------------------------------------------------------------

/// Validate, hash, segment, and scan uploaded files. Documents live in memory.
#[tauri::command]
async fn ingest_files(
    state: State<'_, AppState>,
    session: SessionId,
    files: Vec<IngestFile>,
) -> AppResult<Vec<Document>> {
    extraction::validate_file_count(files.len())?;
    let mut docs = Vec::new();
    let mut findings = Vec::new();
    for (i, file) in files.iter().enumerate() {
        let (doc, mut doc_findings) = ingest_one((i + 1) as u32, file).await?;
        findings.append(&mut doc_findings);
        docs.push(doc);
    }
    state.set_documents(&session.0, docs.clone(), findings)?;
    Ok(docs)
}

/// Ingest a single file into a segmented, scanned [`Document`].
async fn ingest_one(
    index: u32,
    file: &IngestFile,
) -> AppResult<(Document, Vec<crate::models::InjectionFinding>)> {
    let bytes = STANDARD
        .decode(file.data_base64.as_bytes())
        .map_err(|_| AppError::Validation("could not read file data".into()))?;
    let file_type = extraction::validate_upload(&bytes)?;
    let hash = extraction::hash_bytes(&bytes);

    let (pages, source) = match file.kind {
        IngestKind::Pdf => {
            let pages = file
                .pages
                .clone()
                .ok_or_else(|| AppError::Validation("missing extracted PDF pages".into()))?;
            extraction::validate_page_count(pages.len() as u32)?;
            let pages = pages
                .into_iter()
                .map(|p| Page { number: p.number, text: p.text })
                .collect::<Vec<_>>();
            (pages, SourceKind::PdfText)
        }
        IngestKind::Image => {
            let provider = real_provider()?;
            let media = if file_type == extraction::FileType::Png {
                "image/png"
            } else {
                "image/jpeg"
            };
            let text = provider.ocr_image(&file.data_base64, media).await?;
            (vec![Page { number: 1, text }], SourceKind::ImageOcr)
        }
    };

    let mut doc = segmentation::segment_document(
        index,
        base_name(&file.file_name),
        file.role,
        source,
        hash,
        &pages,
    );
    let findings = injection::scan_spans(&mut doc.spans);
    Ok((doc, findings))
}

// ---- Analysis / Q&A / reply -------------------------------------------------

/// Run the analysis pipeline for a session (requires an API key).
#[tauri::command]
async fn run_analysis(state: State<'_, AppState>, session: SessionId) -> AppResult<Analysis> {
    let snap = state.snapshot(&session.0)?;
    if snap.docs.is_empty() {
        return Err(AppError::Validation("no documents to analyse".into()));
    }
    let provider = real_provider()?;
    state.set_provider(&session.0, provider.clone())?;
    let analysis = pipeline::analyze(
        provider.as_ref(),
        &snap.docs,
        snap.anchor,
        today(),
        snap.language,
        snap.findings,
    )
    .await?;
    state.set_analysis(&session.0, analysis.clone())?;
    Ok(analysis)
}

/// Return the cached analysis for a session.
#[tauri::command]
async fn get_analysis(state: State<'_, AppState>, session: SessionId) -> AppResult<Analysis> {
    state.get_analysis(&session.0)
}

/// Answer a grounded question.
#[tauri::command]
async fn ask(
    state: State<'_, AppState>,
    session: SessionId,
    question: String,
) -> AppResult<QaAnswer> {
    let snap = state.snapshot(&session.0)?;
    let provider = match snap.provider {
        Some(p) => p,
        None => real_provider()?,
    };
    pipeline::answer_question(provider.as_ref(), &snap.docs, &question, snap.language).await
}

/// Draft a neutral reply.
#[tauri::command]
async fn draft_reply(state: State<'_, AppState>, session: SessionId) -> AppResult<ReplyDraft> {
    let snap = state.snapshot(&session.0)?;
    let provider = match snap.provider {
        Some(p) => p,
        None => real_provider()?,
    };
    pipeline::draft_reply(provider.as_ref(), &snap.docs, snap.language).await
}

// ---- Demo + delete ----------------------------------------------------------

/// Load a bundled demo scenario (fully offline) and analyse it.
#[tauri::command]
async fn load_demo(
    state: State<'_, AppState>,
    scenario: DemoScenarioKind,
) -> AppResult<DemoSession> {
    let Scenario { docs, findings, provider, anchor, language } = match scenario {
        DemoScenarioKind::Eviction => fixtures::eviction(),
        DemoScenarioKind::Scam => fixtures::scam(),
    };
    let id = new_session_id();
    state.create_session(&id, language, true)?;
    let provider: Arc<dyn LlmProvider> = Arc::new(provider);
    state.set_provider(&id, provider.clone())?;
    state.set_anchor(&id, anchor.clone())?;
    state.set_documents(&id, docs.clone(), findings.clone())?;
    let analysis = pipeline::analyze(
        provider.as_ref(),
        &docs,
        anchor,
        today(),
        language,
        findings,
    )
    .await?;
    state.set_analysis(&id, analysis.clone())?;
    Ok(DemoSession { session_id: SessionId(id), docs, analysis })
}

/// Wipe all in-memory session data.
#[tauri::command]
async fn delete_everything(state: State<'_, AppState>) -> AppResult<()> {
    state.delete_all()
}

/// Build and run the Tauri application.
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            has_api_key,
            set_api_key,
            delete_api_key,
            start_session,
            set_language,
            set_received_date,
            ingest_files,
            run_analysis,
            get_analysis,
            ask,
            draft_reply,
            load_demo,
            delete_everything,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| {
            eprintln!("fatal: failed to start Olai: {e}");
            std::process::exit(1);
        });
}
