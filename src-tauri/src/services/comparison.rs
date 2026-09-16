//! Convert the `compare` model output into numbered, verified conflicts.

use crate::models::Conflict;
use crate::providers::schema::CompareOutput;
use crate::services::analysis::{finalize_claim, scrub};
use std::collections::HashMap;

/// Build numbered conflicts, verifying both the notice and agreement sides.
pub fn build_conflicts(out: CompareOutput, span_text: &HashMap<String, String>) -> Vec<Conflict> {
    out.conflicts
        .into_iter()
        .enumerate()
        .map(|(i, c)| Conflict {
            number: (i + 1) as u32,
            title: scrub(&c.title),
            notice_side: finalize_claim(c.notice, format!("conflict-{i}-notice"), span_text),
            agreement_side: finalize_claim(
                c.agreement,
                format!("conflict-{i}-agreement"),
                span_text,
            ),
            explanation: scrub(&c.explanation),
            severity: c.severity,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Severity, VerificationStatus};

    #[test]
    fn numbers_conflicts_and_verifies_both_sides() {
        let span_text: HashMap<String, String> = [
            (
                "D1-P1-S1".to_string(),
                "You must vacate within 7 days.".to_string(),
            ),
            (
                "D2-P1-S9".to_string(),
                "Clause 9: one month written notice.".to_string(),
            ),
        ]
        .into_iter()
        .collect();
        let out: CompareOutput = serde_json::from_str(
            r#"{"conflicts":[{
                "title":"Notice period is shorter than agreed",
                "notice":{"text":"7 days","citations":[{"spanId":"D1-P1-S1","quote":"vacate within 7 days"}]},
                "agreement":{"text":"one month","citations":[{"spanId":"D2-P1-S9","quote":"one month written notice"}]},
                "explanation":"The notice gives less time than the agreement.",
                "severity":"high"}]}"#,
        )
        .unwrap();
        let conflicts = build_conflicts(out, &span_text);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].number, 1);
        assert_eq!(conflicts[0].severity, Severity::High);
        assert_eq!(
            conflicts[0].notice_side.status,
            VerificationStatus::Verified
        );
        assert_eq!(
            conflicts[0].agreement_side.status,
            VerificationStatus::Verified
        );
    }

    #[test]
    fn scrubs_verdict_in_explanation() {
        let out: CompareOutput = serde_json::from_str(
            r#"{"conflicts":[{"title":"t","notice":{"text":"n"},"agreement":{"text":"a"},
                "explanation":"This notice is not enforceable","severity":"low"}]}"#,
        )
        .unwrap();
        let conflicts = build_conflicts(out, &HashMap::new());
        assert_eq!(
            conflicts[0].explanation,
            crate::services::analysis::VERDICT_REPLACEMENT
        );
    }
}
