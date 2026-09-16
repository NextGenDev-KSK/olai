//! An offline [`LlmProvider`] that replays canned responses by request key.
//!
//! Used by demo mode and every test. Scripted responses (queued per key) let
//! tests exercise the schema-repair retry path deterministically.

use super::{CompletionRequest, LlmProvider};
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

/// Replays responses keyed by [`CompletionRequest::key`].
#[derive(Default)]
pub struct MockProvider {
    /// Default response per key (returned when no scripted response remains).
    defaults: HashMap<String, String>,
    /// Ordered scripted responses per key, consumed one per call.
    scripted: Mutex<HashMap<String, VecDeque<String>>>,
}

impl MockProvider {
    /// Create an empty provider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: set the default response for a key.
    pub fn with(mut self, key: impl Into<String>, response: impl Into<String>) -> Self {
        self.defaults.insert(key.into(), response.into());
        self
    }

    /// Queue a scripted response consumed before the default (FIFO). Useful for
    /// returning invalid JSON first, then valid JSON on the repair retry.
    pub fn push_script(&self, key: impl Into<String>, response: impl Into<String>) {
        if let Ok(mut guard) = self.scripted.lock() {
            guard.entry(key.into()).or_default().push_back(response.into());
        }
    }
}

#[async_trait]
impl LlmProvider for MockProvider {
    async fn complete(&self, req: &CompletionRequest) -> AppResult<String> {
        {
            let mut guard = self
                .scripted
                .lock()
                .map_err(|_| AppError::Provider("mock lock poisoned".into()))?;
            if let Some(queue) = guard.get_mut(&req.key) {
                if let Some(next) = queue.pop_front() {
                    return Ok(next);
                }
            }
        }
        self.defaults
            .get(&req.key)
            .cloned()
            .ok_or_else(|| AppError::Provider(format!("no mock response for key '{}'", req.key)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::{ModelTier, PromptTask};

    fn req(key: &str) -> CompletionRequest {
        CompletionRequest {
            task: PromptTask::Qa,
            tier: ModelTier::Fast,
            key: key.to_string(),
            system: String::new(),
            user: String::new(),
            max_tokens: 100,
        }
    }

    #[tokio::test]
    async fn returns_default_for_key() {
        let p = MockProvider::new().with("qa:x", "{\"ok\":true}");
        assert_eq!(p.complete(&req("qa:x")).await.unwrap(), "{\"ok\":true}");
    }

    #[tokio::test]
    async fn scripted_responses_consumed_before_default() {
        let p = MockProvider::new().with("k", "DEFAULT");
        p.push_script("k", "FIRST");
        p.push_script("k", "SECOND");
        assert_eq!(p.complete(&req("k")).await.unwrap(), "FIRST");
        assert_eq!(p.complete(&req("k")).await.unwrap(), "SECOND");
        assert_eq!(p.complete(&req("k")).await.unwrap(), "DEFAULT");
    }

    #[tokio::test]
    async fn missing_key_is_provider_error() {
        let p = MockProvider::new();
        assert!(matches!(p.complete(&req("nope")).await, Err(AppError::Provider(_))));
    }
}
