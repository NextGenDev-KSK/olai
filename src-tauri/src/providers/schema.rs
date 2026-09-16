//! Strongly-typed shapes for model output. Deserializing into these structs
//! *is* the schema validation step — anything that does not fit is rejected and
//! triggers a single repair retry (see the pipeline). Raw claims are converted
//! into domain [`Claim`]s that are then verified against span text.

use crate::models::{ActionGroup, Citation, Claim, QaStatus, Severity, VerificationStatus};
use serde::Deserialize;

/// Parse the page number out of a `D{d}-P{p}-S{n}` span id (0 if malformed).
pub fn page_from_span_id(id: &str) -> u32 {
    id.split('-')
        .find_map(|part| part.strip_prefix('P'))
        .and_then(|p| p.parse().ok())
        .unwrap_or(0)
}

/// A citation as emitted by the model: a span id and a verbatim quote.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCitation {
    /// The cited span id.
    pub span_id: String,
    /// The verbatim quote.
    pub quote: String,
}

impl RawCitation {
    /// Convert to an unverified [`Citation`] (similarity filled in later).
    pub fn into_citation(self) -> Citation {
        let page = page_from_span_id(&self.span_id);
        Citation {
            span_id: self.span_id,
            quote: self.quote,
            page,
            similarity: 0.0,
            confirmed: false,
        }
    }
}

/// A claim as emitted by the model.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawClaim {
    /// The plain-language claim text.
    pub text: String,
    /// Supporting citations (may be empty).
    #[serde(default)]
    pub citations: Vec<RawCitation>,
}

impl RawClaim {
    /// Convert to an unverified [`Claim`] with a caller-supplied id.
    pub fn into_claim(self, id: impl Into<String>) -> Claim {
        Claim {
            id: id.into(),
            text: self.text,
            citations: self
                .citations
                .into_iter()
                .map(RawCitation::into_citation)
                .collect(),
            status: VerificationStatus::NotConfirmed,
        }
    }
}

/// A party identified by the model.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawParty {
    /// The party's role, e.g. "Landlord".
    pub role: String,
    /// The cited claim naming the party.
    pub claim: RawClaim,
}

/// Output of the `classify` prompt.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifyOutput {
    /// The document type claim.
    pub doc_type: RawClaim,
    /// Parties identified.
    #[serde(default)]
    pub parties: Vec<RawParty>,
    /// One-line summary claim.
    pub summary: RawClaim,
}

/// The unit of a deadline interval extracted by the model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeadlineUnit {
    /// Interval measured in days.
    Days,
    /// Interval measured in months.
    Months,
}

/// A deadline interval extracted by the model. The model NEVER computes the
/// resulting date — only the interval and the clause that states it.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawDeadline {
    /// What the deadline is for.
    pub label: String,
    /// The interval unit.
    pub unit: DeadlineUnit,
    /// The interval amount (non-negative).
    pub amount: i64,
    /// The cited clause stating the interval.
    pub claim: RawClaim,
}

/// Output of the `extract_claims` prompt.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractOutput {
    /// Extracted demand/obligation claims.
    #[serde(default)]
    pub claims: Vec<RawClaim>,
    /// Extracted deadline intervals.
    #[serde(default)]
    pub deadlines: Vec<RawDeadline>,
}

/// A conflict emitted by the model.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawConflict {
    /// Plain-language conflict title.
    pub title: String,
    /// The notice side claim.
    pub notice: RawClaim,
    /// The agreement side claim.
    pub agreement: RawClaim,
    /// Neutral explanation.
    pub explanation: String,
    /// Severity.
    pub severity: Severity,
}

/// Output of the `compare` prompt.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompareOutput {
    /// Conflicts found.
    #[serde(default)]
    pub conflicts: Vec<RawConflict>,
}

/// An action item emitted by the model.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawAction {
    /// The action group.
    pub group: ActionGroup,
    /// The action text.
    pub text: String,
    /// Supporting citations.
    #[serde(default)]
    pub citations: Vec<RawCitation>,
}

/// Output of the `actions` prompt.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionsOutput {
    /// The action checklist.
    #[serde(default)]
    pub actions: Vec<RawAction>,
}

/// Output of the `qa` prompt.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QaOutput {
    /// The answer status.
    pub status: QaStatus,
    /// The answer text.
    pub text: String,
    /// Supporting citations.
    #[serde(default)]
    pub citations: Vec<RawCitation>,
}

/// Output of the `reply` prompt.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyOutput {
    /// The draft reply text.
    pub text: String,
    /// Footnote citations.
    #[serde(default)]
    pub footnotes: Vec<RawCitation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_page_from_span_id() {
        assert_eq!(page_from_span_id("D1-P2-S3"), 2);
        assert_eq!(page_from_span_id("D2-P15-S1"), 15);
        assert_eq!(page_from_span_id("garbage"), 0);
    }

    #[test]
    fn raw_claim_converts_and_starts_unverified() {
        let raw: RawClaim = serde_json::from_str(
            r#"{"text":"You have 7 days","citations":[{"spanId":"D1-P1-S2","quote":"seven days"}]}"#,
        )
        .unwrap();
        let claim = raw.into_claim("c1");
        assert_eq!(claim.status, VerificationStatus::NotConfirmed);
        assert_eq!(claim.citations[0].page, 1);
        assert!(!claim.citations[0].confirmed);
    }

    #[test]
    fn classify_output_deserializes() {
        let out: ClassifyOutput = serde_json::from_str(
            r#"{"docType":{"text":"Eviction notice","citations":[]},
                "parties":[{"role":"Landlord","claim":{"text":"Mr X","citations":[]}}],
                "summary":{"text":"A notice to vacate","citations":[]}}"#,
        )
        .unwrap();
        assert_eq!(out.parties.len(), 1);
        assert_eq!(out.doc_type.text, "Eviction notice");
    }

    #[test]
    fn severity_and_group_use_camel_case() {
        let c: RawConflict = serde_json::from_str(
            r#"{"title":"t","notice":{"text":"n"},"agreement":{"text":"a"},
                "explanation":"e","severity":"high"}"#,
        )
        .unwrap();
        assert_eq!(c.severity, Severity::High);

        let a: RawAction =
            serde_json::from_str(r#"{"group":"getHelp","text":"call aid"}"#).unwrap();
        assert_eq!(a.group, ActionGroup::GetHelp);
    }

    #[test]
    fn extract_output_reads_deadline_units() {
        let out: ExtractOutput = serde_json::from_str(
            r#"{"deadlines":[{"label":"Vacate","unit":"days","amount":7,
                "claim":{"text":"7 days","citations":[]}}]}"#,
        )
        .unwrap();
        assert_eq!(out.deadlines[0].unit, DeadlineUnit::Days);
        assert_eq!(out.deadlines[0].amount, 7);
    }
}
