//! Conversion of model output into verified domain types: the paper card,
//! computed deadlines, verdict scrubbing, and the span-text lookup.

use crate::models::{Claim, Deadline, Document, Language, PaperCard, Party};
use crate::providers::schema::{ClassifyOutput, DeadlineUnit, ExtractOutput, RawClaim};
use crate::services::{dates, safety, verification};
use chrono::NaiveDate;
use std::collections::HashMap;

/// Replacement text shown when the verdict filter removes a statement.
pub const VERDICT_REPLACEMENT: &str =
    "(Removed by Olai's safety filter: this read like a verdict. Olai explains documents but does not judge their validity.)";

/// Build the `span id -> verbatim text` lookup across all documents.
pub fn span_text_map(docs: &[Document]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for doc in docs {
        for span in &doc.spans {
            map.insert(span.id.clone(), span.text.clone());
        }
    }
    map
}

/// Replace text with a neutral notice if it contains a blocked verdict word.
pub fn scrub(text: &str) -> String {
    if safety::find_verdict_word(text).is_some() {
        VERDICT_REPLACEMENT.to_string()
    } else {
        text.to_string()
    }
}

/// Convert a raw claim to a scrubbed, verified [`Claim`].
pub fn finalize_claim(
    raw: RawClaim,
    id: impl Into<String>,
    span_text: &HashMap<String, String>,
) -> Claim {
    let mut claim = raw.into_claim(id);
    claim.text = scrub(&claim.text);
    verification::verify_claim(claim, span_text)
}

/// The instruction appended to make the model reply in the chosen language.
pub fn language_instruction(lang: Language) -> &'static str {
    match lang {
        Language::En => "Write all output text in clear, plain English.",
        Language::Ta => "Write all output text in Tamil (தமிழ்).",
        Language::Hi => "Write all output text in Hindi (हिन्दी).",
    }
}

/// Build the paper card from the `classify` output.
pub fn build_paper_card(out: ClassifyOutput, span_text: &HashMap<String, String>) -> PaperCard {
    let doc_type = finalize_claim(out.doc_type, "docType", span_text);
    let summary = finalize_claim(out.summary, "summary", span_text);
    let parties = out
        .parties
        .into_iter()
        .enumerate()
        .map(|(i, p)| Party {
            role: p.role,
            claim: finalize_claim(p.claim, format!("party-{i}"), span_text),
        })
        .collect();
    PaperCard {
        doc_type,
        parties,
        summary,
    }
}

/// Build computed deadlines from the `extract` output. All date arithmetic is
/// done by [`dates::compute`]; the model only supplies the interval.
pub fn build_deadlines(
    out: &ExtractOutput,
    anchor: Option<(NaiveDate, String)>,
    today: NaiveDate,
    span_text: &HashMap<String, String>,
) -> Vec<Deadline> {
    out.deadlines
        .iter()
        .enumerate()
        .map(|(i, raw)| {
            let interval = match raw.unit {
                DeadlineUnit::Days => dates::Interval::Days(raw.amount),
                DeadlineUnit::Months => dates::Interval::Months(raw.amount),
            };
            let computed = dates::compute(anchor.clone(), interval, today);
            let source = finalize_claim(raw.claim.clone(), format!("deadline-{i}"), span_text);
            Deadline {
                label: raw.label.clone(),
                date: computed.date.map(|d| d.format("%Y-%m-%d").to_string()),
                days_remaining: computed.days_remaining,
                calc: computed.calc,
                source,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::VerificationStatus;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn scrub_replaces_verdict_language() {
        assert_eq!(scrub("This notice is valid"), VERDICT_REPLACEMENT);
        assert_eq!(
            scrub("The notice gives seven days"),
            "The notice gives seven days"
        );
    }

    #[test]
    fn finalize_claim_verifies_against_spans() {
        let span_text = map(&[("D1-P1-S1", "You must vacate within seven days.")]);
        let raw: RawClaim = serde_json::from_str(
            r#"{"text":"Seven days to vacate","citations":[{"spanId":"D1-P1-S1","quote":"vacate within seven days"}]}"#,
        )
        .unwrap();
        let claim = finalize_claim(raw, "c1", &span_text);
        assert_eq!(claim.status, VerificationStatus::Verified);
    }

    #[test]
    fn build_deadlines_reports_missing_anchor() {
        let out: ExtractOutput = serde_json::from_str(
            r#"{"deadlines":[{"label":"Vacate","unit":"days","amount":7,"claim":{"text":"7 days","citations":[]}}]}"#,
        )
        .unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 9, 16).unwrap();
        let deadlines = build_deadlines(&out, None, today, &HashMap::new());
        assert_eq!(deadlines.len(), 1);
        assert!(deadlines[0].calc.needs_anchor);
        assert_eq!(deadlines[0].date, None);
    }

    #[test]
    fn build_deadlines_computes_from_anchor() {
        let out: ExtractOutput = serde_json::from_str(
            r#"{"deadlines":[{"label":"Vacate","unit":"days","amount":7,"claim":{"text":"7 days","citations":[]}}]}"#,
        )
        .unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 9, 16).unwrap();
        let anchor = Some((
            NaiveDate::from_ymd_opt(2026, 9, 10).unwrap(),
            "2026-09-10".to_string(),
        ));
        let deadlines = build_deadlines(&out, anchor, today, &HashMap::new());
        assert_eq!(deadlines[0].date.as_deref(), Some("2026-09-17"));
        assert_eq!(deadlines[0].days_remaining, Some(1));
    }
}
