//! The LLM provider abstraction. `LlmProvider` is dependency-injected so the
//! whole pipeline runs against [`mock::MockProvider`] with no network. The real
//! [`anthropic::AnthropicProvider`] is only compiled with the `app` feature.

pub mod schema;

pub mod mock;

#[cfg(feature = "http")]
pub mod anthropic;

use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Which prompt is being run (selects the versioned template and schema).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromptTask {
    /// Identify the document type and parties.
    Classify,
    /// Extract demands, obligations, and deadline intervals.
    Extract,
    /// Compare the notice against the agreement.
    Compare,
    /// Produce the action checklist.
    Actions,
    /// Draft a neutral reply.
    Reply,
    /// Answer a grounded question.
    Qa,
}

/// Model capability tier. Simple steps use the cheaper, faster model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    /// Cheaper/faster model for simple extraction and Q&A.
    Fast,
    /// More capable model for comparison and drafting.
    Smart,
}

/// A single model completion request.
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    /// The task being run.
    pub task: PromptTask,
    /// The desired model tier.
    pub tier: ModelTier,
    /// A caller-defined key used for caching and mock lookup
    /// (e.g. `"extract:notice"`).
    pub key: String,
    /// The system prompt (a versioned template).
    pub system: String,
    /// The user content (documents wrapped in `<document>` tags).
    pub user: String,
    /// Maximum output tokens.
    pub max_tokens: u32,
}

/// Abstraction over an LLM backend. Implementations must be `Send + Sync` so
/// the pipeline can run independent calls concurrently behind an `Arc`.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Complete the request, returning the raw model text (expected JSON).
    async fn complete(&self, req: &CompletionRequest) -> AppResult<String>;

    /// Transcribe (OCR) an image. The default errors; a vision-capable provider
    /// overrides this.
    async fn ocr_image(&self, _image_base64: &str, _media_type: &str) -> AppResult<String> {
        Err(AppError::Provider(
            "image OCR is not supported by this provider".into(),
        ))
    }
}

/// Extract the JSON object substring (tolerates code fences / surrounding
/// prose) and deserialize it. A parse failure maps to [`AppError::SchemaViolation`].
pub fn parse_json<T: DeserializeOwned>(raw: &str) -> AppResult<T> {
    let slice = json_object_slice(raw).ok_or(AppError::SchemaViolation)?;
    serde_json::from_str(slice).map_err(|_| AppError::SchemaViolation)
}

/// Return the substring from the first `{` to the last `}` inclusive.
fn json_object_slice(raw: &str) -> Option<&str> {
    let start = raw.find('{')?;
    let end = raw.rfind('}')?;
    if end >= start {
        Some(&raw[start..=end])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Sample {
        a: u32,
    }

    #[test]
    fn parses_plain_json() {
        let v: Sample = parse_json(r#"{"a":5}"#).unwrap();
        assert_eq!(v, Sample { a: 5 });
    }

    #[test]
    fn tolerates_code_fences_and_prose() {
        let raw = "Here you go:\n```json\n{\"a\": 7}\n```\nThanks!";
        let v: Sample = parse_json(raw).unwrap();
        assert_eq!(v.a, 7);
    }

    #[test]
    fn rejects_non_json() {
        let err = parse_json::<Sample>("no json here");
        assert!(matches!(err, Err(AppError::SchemaViolation)));
    }

    #[test]
    fn rejects_malformed_json() {
        let err = parse_json::<Sample>(r#"{"a": "not a number"}"#);
        assert!(matches!(err, Err(AppError::SchemaViolation)));
    }
}
