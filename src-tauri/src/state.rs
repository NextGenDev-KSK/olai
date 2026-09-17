//! In-memory session state. Documents live only here for the process lifetime
//! and are never written to disk. The store is pure (no Tauri types) so it is
//! unit-testable; command handlers wrap it in a `tauri::State`.

use crate::error::{AppError, AppResult};
use crate::models::{Analysis, Document, InjectionFinding, Language};
use crate::providers::LlmProvider;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// One user session's in-memory data.
pub struct Session {
    /// The chosen content/UI language.
    pub language: Language,
    /// The optional received-date anchor (ISO or DD/MM).
    pub anchor: Option<String>,
    /// Whether this is a demo session (offline mock provider).
    pub demo: bool,
    /// Ingested documents.
    pub docs: Vec<Document>,
    /// Injection/hidden-text findings from ingestion.
    pub findings: Vec<InjectionFinding>,
    /// Cached analysis result, if computed.
    pub analysis: Option<Analysis>,
    /// The provider used for this session (mock for demo, Anthropic otherwise).
    pub provider: Option<Arc<dyn LlmProvider>>,
}

impl Session {
    fn new(language: Language, demo: bool) -> Self {
        Session {
            language,
            anchor: None,
            demo,
            docs: Vec::new(),
            findings: Vec::new(),
            analysis: None,
            provider: None,
        }
    }
}

/// A cheap, lock-free snapshot used by async command handlers so no lock is
/// held across an `.await`.
pub struct SessionSnapshot {
    /// Documents (cloned).
    pub docs: Vec<Document>,
    /// Findings (cloned).
    pub findings: Vec<InjectionFinding>,
    /// Language.
    pub language: Language,
    /// Anchor date.
    pub anchor: Option<String>,
    /// The session's provider (Arc clone), if set.
    pub provider: Option<Arc<dyn LlmProvider>>,
}

/// The shared application state: a map of sessions behind a lock.
#[derive(Default)]
pub struct AppState {
    sessions: RwLock<HashMap<String, Session>>,
}

impl AppState {
    /// Create an empty state.
    pub fn new() -> Self {
        Self::default()
    }

    fn write(&self) -> AppResult<std::sync::RwLockWriteGuard<'_, HashMap<String, Session>>> {
        self.sessions
            .write()
            .map_err(|_| AppError::Internal("state lock".into()))
    }

    fn read(&self) -> AppResult<std::sync::RwLockReadGuard<'_, HashMap<String, Session>>> {
        self.sessions
            .read()
            .map_err(|_| AppError::Internal("state lock".into()))
    }

    fn session_missing(id: &str) -> AppError {
        AppError::NotFound(format!("session {id}"))
    }

    /// Create a new session with the given id and language.
    pub fn create_session(&self, id: &str, language: Language, demo: bool) -> AppResult<()> {
        self.write()?
            .insert(id.to_string(), Session::new(language, demo));
        Ok(())
    }

    /// Mutate a session in place.
    fn with_mut<R>(&self, id: &str, f: impl FnOnce(&mut Session) -> R) -> AppResult<R> {
        let mut guard = self.write()?;
        let session = guard.get_mut(id).ok_or_else(|| Self::session_missing(id))?;
        Ok(f(session))
    }

    /// Set the session language.
    pub fn set_language(&self, id: &str, language: Language) -> AppResult<()> {
        self.with_mut(id, |s| s.language = language)
    }

    /// Set the received-date anchor.
    pub fn set_anchor(&self, id: &str, anchor: Option<String>) -> AppResult<()> {
        self.with_mut(id, |s| s.anchor = anchor)
    }

    /// Store ingested documents and their findings.
    pub fn set_documents(
        &self,
        id: &str,
        docs: Vec<Document>,
        findings: Vec<InjectionFinding>,
    ) -> AppResult<()> {
        self.with_mut(id, |s| {
            s.docs = docs;
            s.findings = findings;
            s.analysis = None;
        })
    }

    /// Set the provider used for this session.
    pub fn set_provider(&self, id: &str, provider: Arc<dyn LlmProvider>) -> AppResult<()> {
        self.with_mut(id, |s| s.provider = Some(provider))
    }

    /// Cache the analysis result.
    pub fn set_analysis(&self, id: &str, analysis: Analysis) -> AppResult<()> {
        self.with_mut(id, |s| s.analysis = Some(analysis))
    }

    /// Take a snapshot for async work.
    pub fn snapshot(&self, id: &str) -> AppResult<SessionSnapshot> {
        let guard = self.read()?;
        let s = guard.get(id).ok_or_else(|| Self::session_missing(id))?;
        Ok(SessionSnapshot {
            docs: s.docs.clone(),
            findings: s.findings.clone(),
            language: s.language,
            anchor: s.anchor.clone(),
            provider: s.provider.clone(),
        })
    }

    /// Get the cached analysis, or an error if none has been computed.
    pub fn get_analysis(&self, id: &str) -> AppResult<Analysis> {
        let guard = self.read()?;
        let s = guard.get(id).ok_or_else(|| Self::session_missing(id))?;
        s.analysis
            .clone()
            .ok_or_else(|| AppError::NotFound("analysis".into()))
    }

    /// Remove a single session.
    pub fn delete_session(&self, id: &str) -> AppResult<()> {
        self.write()?.remove(id);
        Ok(())
    }

    /// Wipe all session data (the "Delete everything" action).
    pub fn delete_all(&self) -> AppResult<()> {
        self.write()?.clear();
        Ok(())
    }

    /// Number of active sessions (used in tests).
    pub fn count(&self) -> AppResult<usize> {
        Ok(self.read()?.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::mock::MockProvider;

    #[test]
    fn create_snapshot_and_delete() {
        let state = AppState::new();
        state.create_session("s1", Language::Ta, true).unwrap();
        state.set_anchor("s1", Some("2026-09-10".into())).unwrap();
        state
            .set_provider("s1", Arc::new(MockProvider::new()))
            .unwrap();

        let snap = state.snapshot("s1").unwrap();
        assert_eq!(snap.language, Language::Ta);
        assert_eq!(snap.anchor.as_deref(), Some("2026-09-10"));
        assert!(snap.provider.is_some());

        assert_eq!(state.count().unwrap(), 1);
        state.delete_all().unwrap();
        assert_eq!(state.count().unwrap(), 0);
    }

    #[test]
    fn missing_session_is_not_found() {
        let state = AppState::new();
        assert!(matches!(state.snapshot("nope"), Err(AppError::NotFound(_))));
        assert!(matches!(
            state.get_analysis("nope"),
            Err(AppError::NotFound(_))
        ));
    }
}
