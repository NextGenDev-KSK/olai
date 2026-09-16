//! The analysis orchestrator. Runs the independent model calls concurrently,
//! validates and repairs schema, verifies citations, computes deadlines, and
//! assembles the safety-tiered [`Analysis`]. Also hosts grounded Q&A and reply.

use crate::error::AppResult;
use crate::models::Document;
use crate::models::{
    Analysis, HelpTier, InjectionFinding, Language, QaAnswer, QaStatus, ReplyDraft,
};
use crate::prompts;
use crate::providers::schema::{
    ActionsOutput, ClassifyOutput, CompareOutput, ExtractOutput, QaOutput, ReplyOutput,
};
use crate::providers::{parse_json, CompletionRequest, LlmProvider, ModelTier, PromptTask};
use crate::services::{actions, analysis, comparison, dates, safety, verification};
use chrono::NaiveDate;
use serde::de::DeserializeOwned;

/// Maximum output tokens per call.
const MAX_TOKENS: u32 = 2048;

/// Assemble a completion request.
fn make_req(task: PromptTask, tier: ModelTier, key: &str, user: String) -> CompletionRequest {
    CompletionRequest {
        task,
        tier,
        key: key.to_string(),
        system: prompts::system_for(task),
        user,
        max_tokens: MAX_TOKENS,
    }
}

/// Compose the user message: documents block + optional extra + language note.
fn user_for(doc_block: &str, lang: Language, extra: Option<&str>) -> String {
    let mut user = doc_block.to_string();
    if let Some(extra) = extra {
        user.push_str("\n\n");
        user.push_str(extra);
    }
    user.push_str("\n\n");
    user.push_str(analysis::language_instruction(lang));
    user
}

/// Call the provider, validating the JSON; on failure, retry once with a repair
/// message before giving up with [`crate::error::AppError::SchemaViolation`].
async fn complete_and_parse<T: DeserializeOwned>(
    provider: &dyn LlmProvider,
    req: &CompletionRequest,
) -> AppResult<T> {
    let first = provider.complete(req).await?;
    if let Ok(value) = parse_json::<T>(&first) {
        return Ok(value);
    }
    let repair = CompletionRequest {
        user: format!("{}\n\n{}", req.user, prompts::REPAIR_MESSAGE),
        ..req.clone()
    };
    let second = provider.complete(&repair).await?;
    parse_json::<T>(&second)
}

/// Run the full analysis pipeline over the ingested documents.
pub async fn analyze(
    provider: &dyn LlmProvider,
    docs: &[Document],
    anchor_raw: Option<String>,
    today: NaiveDate,
    lang: Language,
    findings: Vec<InjectionFinding>,
) -> AppResult<Analysis> {
    let span_text = analysis::span_text_map(docs);
    let doc_block = prompts::render_documents(docs);
    let anchor = anchor_raw
        .as_ref()
        .and_then(|s| dates::parse_anchor(s).map(|d| (d, s.clone())));

    let classify_req = make_req(
        PromptTask::Classify,
        ModelTier::Fast,
        "classify",
        user_for(&doc_block, lang, None),
    );
    let extract_req = make_req(
        PromptTask::Extract,
        ModelTier::Fast,
        "extract:notice",
        user_for(&doc_block, lang, None),
    );
    let compare_req = make_req(
        PromptTask::Compare,
        ModelTier::Smart,
        "compare",
        user_for(&doc_block, lang, None),
    );
    let actions_req = make_req(
        PromptTask::Actions,
        ModelTier::Smart,
        "actions",
        user_for(&doc_block, lang, None),
    );

    // Independent calls run concurrently.
    let (classify, extract, compare, actions_out) = tokio::join!(
        complete_and_parse::<ClassifyOutput>(provider, &classify_req),
        complete_and_parse::<ExtractOutput>(provider, &extract_req),
        complete_and_parse::<CompareOutput>(provider, &compare_req),
        complete_and_parse::<ActionsOutput>(provider, &actions_req),
    );

    let paper_card = analysis::build_paper_card(classify?, &span_text);
    let extract = extract?;
    let deadlines = analysis::build_deadlines(&extract, anchor, today, &span_text);
    let conflicts = comparison::build_conflicts(compare?, &span_text);
    let action_items = actions::build_actions(actions_out?, &span_text);

    let tier = decide_tier(docs, &deadlines);
    let missing_info = collect_missing_info(&deadlines);

    Ok(Analysis {
        tier,
        paper_card,
        deadlines,
        conflicts,
        actions: action_items,
        findings,
        missing_info,
    })
}

/// Decide the help tier from document signals and the soonest deadline.
fn decide_tier(
    docs: &[Document],
    deadlines: &[crate::models::Deadline],
) -> crate::models::TierDecision {
    let doc_text = docs
        .iter()
        .flat_map(|d| d.spans.iter())
        .map(|s| s.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let triggers = safety::Triggers {
        court_or_police: safety::detect_court_or_police(&doc_text),
        min_days_remaining: deadlines.iter().filter_map(|d| d.days_remaining).min(),
        scam: safety::detect_scam_signals(&doc_text),
        strategy: false,
    };
    safety::decide_tier(HelpTier::GeneralGuidance, &triggers)
}

/// Build "missing info" prompts for any deadline lacking an anchor date.
fn collect_missing_info(deadlines: &[crate::models::Deadline]) -> Vec<String> {
    deadlines
        .iter()
        .filter(|d| d.calc.needs_anchor)
        .map(|d| {
            format!(
                "To calculate the \"{}\" deadline, Olai needs the date you received the notice.",
                d.label
            )
        })
        .collect()
}

/// Answer a question grounded strictly in the documents.
pub async fn answer_question(
    provider: &dyn LlmProvider,
    docs: &[Document],
    question: &str,
    lang: Language,
) -> AppResult<QaAnswer> {
    // Strategy/outcome questions are refused before any model call.
    if safety::is_strategy_question(question) {
        return Ok(QaAnswer {
            status: QaStatus::NeedsLawyer,
            text: "This asks about strategy or the likely outcome. Olai can't advise on that — please speak to a lawyer or a free legal-aid service.".to_string(),
            citations: Vec::new(),
        });
    }

    let span_text = analysis::span_text_map(docs);
    let doc_block = prompts::render_documents(docs);
    let extra = format!("QUESTION: {question}");
    let req = make_req(
        PromptTask::Qa,
        ModelTier::Fast,
        "qa",
        user_for(&doc_block, lang, Some(&extra)),
    );
    let out: QaOutput = complete_and_parse(provider, &req).await?;

    let citations = verification::verify_citations(
        out.citations
            .into_iter()
            .map(|c| c.into_citation())
            .collect(),
        &span_text,
    );
    // Enforce grounding: an "answered" reply without fully-verified citations is
    // downgraded so nothing unverified is shown as fact.
    let grounded = !citations.is_empty() && citations.iter().all(|c| c.confirmed);
    match out.status {
        QaStatus::Answered if grounded => Ok(QaAnswer {
            status: QaStatus::Answered,
            text: analysis::scrub(&out.text),
            citations,
        }),
        QaStatus::Answered => Ok(QaAnswer {
            status: QaStatus::NotInDocuments,
            text:
                "Your documents don't say this. You could ask the sender for written clarification."
                    .to_string(),
            citations: Vec::new(),
        }),
        QaStatus::NotInDocuments => Ok(QaAnswer {
            status: QaStatus::NotInDocuments,
            text: analysis::scrub(&out.text),
            citations: Vec::new(),
        }),
        QaStatus::NeedsLawyer => Ok(QaAnswer {
            status: QaStatus::NeedsLawyer,
            text: analysis::scrub(&out.text),
            citations: Vec::new(),
        }),
    }
}

/// Draft a neutral reply with verified footnote citations.
pub async fn draft_reply(
    provider: &dyn LlmProvider,
    docs: &[Document],
    lang: Language,
) -> AppResult<ReplyDraft> {
    let span_text = analysis::span_text_map(docs);
    let doc_block = prompts::render_documents(docs);
    let req = make_req(
        PromptTask::Reply,
        ModelTier::Smart,
        "reply",
        user_for(&doc_block, lang, None),
    );
    let out: ReplyOutput = complete_and_parse(provider, &req).await?;
    let footnotes = verification::verify_citations(
        out.footnotes
            .into_iter()
            .map(|c| c.into_citation())
            .collect(),
        &span_text,
    );
    Ok(ReplyDraft {
        text: analysis::scrub(&out.text),
        footnotes,
    })
}
