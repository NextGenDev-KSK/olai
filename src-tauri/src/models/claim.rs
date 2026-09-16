//! Claims and their citations. Every model claim must cite spans and be
//! verified against the stored span text before the UI may render it as fact.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// The verification status of a claim after Rust checks its citations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum VerificationStatus {
    /// All citations matched their span text with similarity >= threshold.
    Verified,
    /// One or more citations could not be confirmed; render greyed out.
    NotConfirmed,
}

/// A reference from a claim to a specific span, with the verbatim quote.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    /// The cited span id, e.g. `D1-P2-S3`.
    pub span_id: String,
    /// Verbatim quote from the span (at most 25 words).
    pub quote: String,
    /// 1-based page number, denormalized for display.
    pub page: u32,
    /// Similarity score in `[0, 1]` between the quote and the stored span.
    pub similarity: f32,
    /// Whether this individual citation was confirmed.
    pub confirmed: bool,
}

/// A plain-language statement the model made about the documents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    /// Stable id for cross-referencing (e.g. from a conflict).
    pub id: String,
    /// The plain-language claim text.
    pub text: String,
    /// Supporting citations.
    pub citations: Vec<Citation>,
    /// Overall verification status derived from the citations.
    pub status: VerificationStatus,
}

impl Claim {
    /// True when the claim may be shown to the user as a confirmed fact.
    pub fn is_verified(&self) -> bool {
        matches!(self.status, VerificationStatus::Verified)
    }
}
