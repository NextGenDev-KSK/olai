//! Convert the `actions` model output into verified, grouped checklist items.

use crate::models::ActionItem;
use crate::providers::schema::ActionsOutput;
use crate::services::analysis::scrub;
use crate::services::verification;
use std::collections::HashMap;

/// Build the action checklist, scrubbing verdicts and verifying any citations.
pub fn build_actions(out: ActionsOutput, span_text: &HashMap<String, String>) -> Vec<ActionItem> {
    out.actions
        .into_iter()
        .map(|a| {
            let citations = verification::verify_citations(
                a.citations.into_iter().map(|c| c.into_citation()).collect(),
                span_text,
            );
            ActionItem {
                group: a.group,
                text: scrub(&a.text),
                citations,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ActionGroup;

    #[test]
    fn builds_grouped_actions_with_verified_citations() {
        let span_text: HashMap<String, String> = [(
            "D1-P1-S1".to_string(),
            "Deposit of Rs 60,000 is forfeited.".to_string(),
        )]
        .into_iter()
        .collect();
        let out: ActionsOutput = serde_json::from_str(
            r#"{"actions":[
                {"group":"protect","text":"Keep a copy of every document.","citations":[]},
                {"group":"gather","text":"Note the deposit amount.","citations":[{"spanId":"D1-P1-S1","quote":"Deposit of Rs 60,000 is forfeited"}]}
            ]}"#,
        )
        .unwrap();
        let items = build_actions(out, &span_text);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].group, ActionGroup::Protect);
        assert!(items[1].citations[0].confirmed);
    }
}
