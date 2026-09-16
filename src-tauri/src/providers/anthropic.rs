//! The real Anthropic Messages API provider. Compiled only with the `app`
//! feature. The API key lives here in memory for the process lifetime and is
//! never logged; network errors are surfaced without leaking the key or URL.

use super::{CompletionRequest, LlmProvider, ModelTier};
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::time::Duration;

/// Anthropic Messages endpoint.
const API_URL: &str = "https://api.anthropic.com/v1/messages";
/// Pinned Anthropic API version.
const API_VERSION: &str = "2023-06-01";
/// Hard per-call timeout.
const TIMEOUT: Duration = Duration::from_secs(45);

/// Calls the Anthropic Messages API over HTTPS (rustls).
pub struct AnthropicProvider {
    api_key: String,
    client: reqwest::Client,
}

impl AnthropicProvider {
    /// Construct a provider with the given API key.
    pub fn new(api_key: String) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(TIMEOUT)
            .https_only(true)
            .build()
            .map_err(|_| AppError::Provider("failed to build HTTP client".into()))?;
        Ok(Self { api_key, client })
    }

    /// Map a capability tier to a concrete model id.
    fn model_id(tier: ModelTier) -> &'static str {
        match tier {
            ModelTier::Fast => "claude-haiku-4-5",
            ModelTier::Smart => "claude-sonnet-5",
        }
    }

    /// Build the request body. Temperature is 0 for deterministic extraction.
    fn build_body(req: &CompletionRequest) -> Value {
        json!({
            "model": Self::model_id(req.tier),
            "max_tokens": req.max_tokens,
            "temperature": 0,
            "system": req.system,
            "messages": [{ "role": "user", "content": req.user }],
        })
    }

    /// Pull the assistant text out of a successful response body.
    fn parse_response(body: &Value) -> AppResult<String> {
        body.get("content")
            .and_then(|c| c.as_array())
            .and_then(|blocks| blocks.first())
            .and_then(|b| b.get("text"))
            .and_then(|t| t.as_str())
            .map(str::to_string)
            .ok_or_else(|| AppError::Provider("unexpected response shape".into()))
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn complete(&self, req: &CompletionRequest) -> AppResult<String> {
        let response = self
            .client
            .post(API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .header("content-type", "application/json")
            .json(&Self::build_body(req))
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AppError::Timeout
                } else {
                    // Deliberately generic: never leak the key or URL.
                    AppError::Provider("network error".into())
                }
            })?;

        if !response.status().is_success() {
            return Err(AppError::Provider(format!(
                "HTTP {}",
                response.status().as_u16()
            )));
        }

        let body: Value = response
            .json()
            .await
            .map_err(|_| AppError::Provider("invalid JSON response".into()))?;
        Self::parse_response(&body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::PromptTask;

    fn req(tier: ModelTier) -> CompletionRequest {
        CompletionRequest {
            task: PromptTask::Classify,
            tier,
            key: "k".into(),
            system: "SYS".into(),
            user: "USR".into(),
            max_tokens: 512,
        }
    }

    #[test]
    fn model_ids_map_by_tier() {
        assert_eq!(
            AnthropicProvider::model_id(ModelTier::Fast),
            "claude-haiku-4-5"
        );
        assert_eq!(
            AnthropicProvider::model_id(ModelTier::Smart),
            "claude-sonnet-5"
        );
    }

    #[test]
    fn build_body_includes_prompt_and_model() {
        let body = AnthropicProvider::build_body(&req(ModelTier::Smart));
        assert_eq!(body["model"], "claude-sonnet-5");
        assert_eq!(body["system"], "SYS");
        assert_eq!(body["messages"][0]["content"], "USR");
        assert_eq!(body["max_tokens"], 512);
        assert_eq!(body["temperature"], 0);
    }

    #[test]
    fn parse_response_reads_first_text_block() {
        let body = json!({ "content": [{ "type": "text", "text": "hello" }] });
        assert_eq!(AnthropicProvider::parse_response(&body).unwrap(), "hello");
    }

    #[test]
    fn parse_response_rejects_bad_shape() {
        let body = json!({ "error": "nope" });
        assert!(AnthropicProvider::parse_response(&body).is_err());
    }
}
