//! Safety-related types: help tiers, PII spans, and injection flags.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// The three labelled help tiers shown to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum HelpTier {
    /// Neutral factual information about the documents.
    Information,
    /// General, non-personalized guidance.
    GeneralGuidance,
    /// The situation needs a lawyer or legal aid — forced in risky cases.
    NeedsLawyer,
}

/// Why a particular help tier was chosen (shown to the user for transparency).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum TierReason {
    /// A court or police document was detected.
    CourtOrPolice,
    /// A deadline falls within seven days.
    ShortDeadline,
    /// Scam signals were detected.
    ScamSignal,
    /// The user asked for strategy or an outcome prediction.
    StrategyQuestion,
    /// No escalation trigger fired; the base tier applies.
    Baseline,
}

/// A category of personally identifiable information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum PiiKind {
    /// Aadhaar-like 12-digit identifier.
    Aadhaar,
    /// Indian PAN (permanent account number).
    Pan,
    /// Phone number.
    Phone,
    /// Email address.
    Email,
    /// Bank account number.
    BankAccount,
    /// UPI virtual payment address.
    Upi,
}

/// A detected PII occurrence, by character range, for masking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct PiiSpan {
    /// The kind of PII detected.
    pub kind: PiiKind,
    /// Byte start offset within the analysed text.
    pub start: usize,
    /// Byte end offset (exclusive).
    pub end: usize,
    /// The masked replacement, e.g. `XXXX-XXXX-1234`.
    pub masked: String,
}

/// A flagged prompt-injection or hidden-text finding for user review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct InjectionFinding {
    /// The span id where the finding occurred.
    pub span_id: String,
    /// Short human-readable reason (e.g. "instruction-like phrase").
    pub reason: String,
    /// The offending excerpt (already PII-masked, at most a few words).
    pub excerpt: String,
}
