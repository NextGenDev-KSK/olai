//! Typed application errors and their mapping to localized, user-facing messages.
//!
//! No `unwrap`/`expect` is used outside tests. Every fallible operation returns
//! [`AppError`]; Tauri commands convert it into a [`UserFacingError`] whose
//! `message_key` the frontend resolves against its i18n catalog.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// All recoverable failures in the Olai backend.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// A document (or other input) failed validation (size, type, count, id).
    #[error("invalid input: {0}")]
    Validation(String),

    /// An uploaded file's magic bytes did not match its claimed type.
    #[error("unsupported or corrupt file")]
    UnsupportedFile,

    /// A limit was exceeded (files, pages, or bytes).
    #[error("limit exceeded: {0}")]
    LimitExceeded(String),

    /// A referenced session, document, or span id does not exist.
    #[error("not found: {0}")]
    NotFound(String),

    /// The Anthropic API key is missing from the credential store.
    #[error("no API key configured")]
    MissingApiKey,

    /// The credential store (Windows Credential Manager) failed.
    #[error("secret storage error")]
    SecretStore,

    /// The model provider failed (network, timeout, HTTP status).
    #[error("model provider error: {0}")]
    Provider(String),

    /// The model call was cancelled by the user.
    #[error("operation cancelled")]
    Cancelled,

    /// The model call exceeded the configured timeout.
    #[error("model call timed out")]
    Timeout,

    /// Model output failed schema validation even after one repair attempt.
    #[error("model output did not match the required schema")]
    SchemaViolation,

    /// The model produced blocked verdict-style language.
    #[error("model output blocked by safety filter")]
    SafetyBlocked,

    /// An internal invariant was violated (a bug, surfaced safely).
    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    /// Stable machine code used by the frontend to pick an i18n message.
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Validation(_) => "error.validation",
            AppError::UnsupportedFile => "error.unsupportedFile",
            AppError::LimitExceeded(_) => "error.limitExceeded",
            AppError::NotFound(_) => "error.notFound",
            AppError::MissingApiKey => "error.missingApiKey",
            AppError::SecretStore => "error.secretStore",
            AppError::Provider(_) => "error.provider",
            AppError::Cancelled => "error.cancelled",
            AppError::Timeout => "error.timeout",
            AppError::SchemaViolation => "error.schema",
            AppError::SafetyBlocked => "error.safetyBlocked",
            AppError::Internal(_) => "error.internal",
        }
    }

    /// Whether a details string is safe to surface (never contains document text).
    fn detail(&self) -> Option<String> {
        match self {
            AppError::Validation(d)
            | AppError::LimitExceeded(d)
            | AppError::NotFound(d)
            | AppError::Provider(d)
            | AppError::Internal(d) => Some(d.clone()),
            _ => None,
        }
    }

    /// Convert to the serializable shape returned across the IPC boundary.
    pub fn to_user_facing(&self) -> UserFacingError {
        UserFacingError {
            code: self.code().to_string(),
            detail: self.detail(),
        }
    }
}

/// Serializable, localizable error returned to the frontend.
///
/// `code` maps to an i18n key; `detail` is an optional, PII-free clarifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct UserFacingError {
    /// Stable i18n key, e.g. `error.limitExceeded`.
    pub code: String,
    /// Optional PII-free clarifier (never contains document text or secrets).
    pub detail: Option<String>,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_user_facing().serialize(serializer)
    }
}

/// Convenience alias used throughout the backend.
pub type AppResult<T> = Result<T, AppError>;

#[cfg(feature = "app")]
impl From<keyring::Error> for AppError {
    fn from(_: keyring::Error) -> Self {
        AppError::SecretStore
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_variant_has_stable_code() {
        let cases = [
            (AppError::Validation("x".into()), "error.validation"),
            (AppError::UnsupportedFile, "error.unsupportedFile"),
            (AppError::LimitExceeded("x".into()), "error.limitExceeded"),
            (AppError::NotFound("x".into()), "error.notFound"),
            (AppError::MissingApiKey, "error.missingApiKey"),
            (AppError::SecretStore, "error.secretStore"),
            (AppError::Provider("x".into()), "error.provider"),
            (AppError::Cancelled, "error.cancelled"),
            (AppError::Timeout, "error.timeout"),
            (AppError::SchemaViolation, "error.schema"),
            (AppError::SafetyBlocked, "error.safetyBlocked"),
            (AppError::Internal("x".into()), "error.internal"),
        ];
        for (err, code) in cases {
            assert_eq!(err.code(), code);
        }
    }

    #[test]
    fn detail_is_hidden_for_opaque_errors() {
        assert_eq!(AppError::MissingApiKey.to_user_facing().detail, None);
        assert_eq!(
            AppError::Validation("too big".into())
                .to_user_facing()
                .detail,
            Some("too big".to_string())
        );
    }

    #[cfg(feature = "app")]
    #[test]
    fn keyring_error_maps_to_secret_store() {
        let e: AppError = keyring::Error::NoEntry.into();
        assert!(matches!(e, AppError::SecretStore));
    }
}
