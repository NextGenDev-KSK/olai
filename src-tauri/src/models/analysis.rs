//! Analysis result types: paper card, deadlines, conflicts, actions, Q&A.

use super::claim::Claim;
use super::safety::{HelpTier, InjectionFinding, TierReason};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Ordered severity for a conflict. Never conveyed by colour alone in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    /// Minor or informational mismatch.
    Low,
    /// A meaningful discrepancy.
    Medium,
    /// A serious discrepancy the user should not ignore.
    High,
}

/// A named party identified in the documents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Party {
    /// The party's role, e.g. "Landlord" or "Tenant".
    pub role: String,
    /// The cited claim naming the party (PII-masked as appropriate).
    pub claim: Claim,
}

/// The "Paper Card": a plain-language summary of what a document is.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct PaperCard {
    /// Document type in plain language, e.g. "Eviction notice".
    pub doc_type: Claim,
    /// Identified parties.
    pub parties: Vec<Party>,
    /// One-line plain summary.
    pub summary: Claim,
}

/// How a deadline date was computed, shown to the user step by step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct DeadlineCalc {
    /// Anchor date in ISO `YYYY-MM-DD`, or `None` if unknown.
    pub anchor: Option<String>,
    /// Human-readable arithmetic, e.g. "2026-09-10 + 7 days".
    pub steps: Vec<String>,
    /// Whether the anchor date is missing and must be asked of the user.
    pub needs_anchor: bool,
}

/// A computed deadline. All arithmetic is done in Rust (chrono), never the model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Deadline {
    /// What the deadline is for, e.g. "Vacate the premises".
    pub label: String,
    /// The computed date in ISO `YYYY-MM-DD`, if the anchor was known.
    pub date: Option<String>,
    /// Days remaining from "today", if computable (may be negative).
    pub days_remaining: Option<i64>,
    /// How the date was derived.
    pub calc: DeadlineCalc,
    /// The claim (with citation) that stated the interval/date.
    pub source: Claim,
}

/// A conflict between the notice and the agreement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Conflict {
    /// 1-based conflict number for the UI.
    pub number: u32,
    /// Plain-language title, e.g. "Notice period is shorter than agreed".
    pub title: String,
    /// What the notice says, with citation.
    pub notice_side: Claim,
    /// What the agreement says, with citation.
    pub agreement_side: Claim,
    /// Neutral explanation of the discrepancy (no verdicts).
    pub explanation: String,
    /// Severity, always paired with an icon and word in the UI.
    pub severity: Severity,
}

/// The four action groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum ActionGroup {
    /// Protect your position.
    Protect,
    /// Gather evidence and documents.
    Gather,
    /// Respond appropriately.
    Respond,
    /// Get help from legal aid.
    GetHelp,
}

/// A single, cited checklist item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct ActionItem {
    /// Which group this belongs to.
    pub group: ActionGroup,
    /// The action, in plain language.
    pub text: String,
    /// Supporting citations (may be empty for generic protective steps).
    pub citations: Vec<super::claim::Citation>,
}

/// The status of a grounded Q&A answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum QaStatus {
    /// Answered from the documents.
    Answered,
    /// The documents do not contain the answer.
    NotInDocuments,
    /// The question needs a lawyer (strategy/outcome).
    NeedsLawyer,
}

/// A grounded answer to a user question.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct QaAnswer {
    /// The answer status pill.
    pub status: QaStatus,
    /// The answer text (empty guidance when not answerable).
    pub text: String,
    /// Supporting citations.
    pub citations: Vec<super::claim::Citation>,
}

/// A drafted neutral reply with footnote citations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct ReplyDraft {
    /// The draft reply text.
    pub text: String,
    /// Footnote citations supporting factual references.
    pub footnotes: Vec<super::claim::Citation>,
}

/// The chosen help tier plus the reasons that produced it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct TierDecision {
    /// The resulting tier.
    pub tier: HelpTier,
    /// The reasons that contributed (for transparency).
    pub reasons: Vec<TierReason>,
}

/// The complete analysis result rendered on the dashboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Analysis {
    /// Help-tier banner decision.
    pub tier: TierDecision,
    /// Paper card for the notice (the primary document).
    pub paper_card: PaperCard,
    /// Computed deadlines.
    pub deadlines: Vec<Deadline>,
    /// Numbered conflicts.
    pub conflicts: Vec<Conflict>,
    /// Action checklist.
    pub actions: Vec<ActionItem>,
    /// Injection/hidden-text findings surfaced for the user.
    pub findings: Vec<InjectionFinding>,
    /// Questions the app still needs answered (e.g. a missing anchor date).
    pub missing_info: Vec<String>,
}
