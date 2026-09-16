//! Integration tests: the full analysis pipeline over the demo scenarios using
//! the offline MockProvider. Asserts conflict counts, forced help tiers, and
//! that nothing unverified is presented as fact.

use chrono::NaiveDate;
use olai_lib::fixtures;
use olai_lib::models::{HelpTier, Language, QaStatus, Severity, TierReason, VerificationStatus};
use olai_lib::providers::mock::MockProvider;
use olai_lib::services::pipeline;
use serde_json::json;

fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 9, 16).expect("valid date")
}

#[tokio::test]
async fn eviction_demo_produces_three_conflicts_and_escalates() {
    let s = fixtures::eviction();
    let analysis = pipeline::analyze(
        &s.provider,
        &s.docs,
        s.anchor.clone(),
        today(),
        s.language,
        s.findings.clone(),
    )
    .await
    .expect("analysis succeeds");

    assert_eq!(
        analysis.conflicts.len(),
        3,
        "notice vs agreement yields three conflicts"
    );
    assert_eq!(analysis.tier.tier, HelpTier::NeedsLawyer);
    assert!(analysis.tier.reasons.contains(&TierReason::ShortDeadline));

    // The deadline is computed in code, not by the model.
    let deadline = &analysis.deadlines[0];
    assert_eq!(deadline.date.as_deref(), Some("2026-09-17"));
    assert_eq!(deadline.days_remaining, Some(1));
    assert!(!deadline.calc.needs_anchor);

    // Nothing unverified is presented as fact.
    for conflict in &analysis.conflicts {
        assert_eq!(conflict.notice_side.status, VerificationStatus::Verified);
        assert_eq!(conflict.agreement_side.status, VerificationStatus::Verified);
    }
    assert_eq!(
        analysis.paper_card.doc_type.status,
        VerificationStatus::Verified
    );
    assert!(analysis
        .conflicts
        .iter()
        .any(|c| c.severity == Severity::High));
}

#[tokio::test]
async fn eviction_qa_is_grounded_in_documents() {
    let s = fixtures::eviction();
    let answer = pipeline::answer_question(
        &s.provider,
        &s.docs,
        "What notice period does the agreement require?",
        s.language,
    )
    .await
    .expect("qa succeeds");

    assert_eq!(answer.status, QaStatus::Answered);
    assert!(!answer.citations.is_empty());
    assert!(answer.citations.iter().all(|c| c.confirmed));
}

#[tokio::test]
async fn strategy_question_is_refused_without_a_model_call() {
    let s = fixtures::eviction();
    let answer = pipeline::answer_question(
        &s.provider,
        &s.docs,
        "Will I win if I fight this in court?",
        s.language,
    )
    .await
    .expect("qa succeeds");

    assert_eq!(answer.status, QaStatus::NeedsLawyer);
    assert!(answer.citations.is_empty());
}

#[tokio::test]
async fn reply_footnotes_are_verified() {
    let s = fixtures::eviction();
    let reply = pipeline::draft_reply(&s.provider, &s.docs, s.language)
        .await
        .expect("reply succeeds");

    assert!(!reply.text.is_empty());
    assert!(reply.footnotes.iter().all(|c| c.confirmed));
}

#[tokio::test]
async fn scam_demo_escalates_and_flags_injection() {
    let s = fixtures::scam();
    assert!(
        !s.findings.is_empty(),
        "hidden text + injection line are detected"
    );

    let analysis = pipeline::analyze(
        &s.provider,
        &s.docs,
        s.anchor.clone(),
        today(),
        s.language,
        s.findings.clone(),
    )
    .await
    .expect("analysis succeeds");

    assert_eq!(analysis.tier.tier, HelpTier::NeedsLawyer);
    assert!(analysis.tier.reasons.contains(&TierReason::ScamSignal));
    assert!(
        analysis.conflicts.is_empty(),
        "no agreement means no conflicts"
    );
    assert!(!analysis.findings.is_empty());
}

#[tokio::test]
async fn fabricated_citation_is_marked_not_confirmed() {
    let docs = fixtures::eviction_docs();
    let mut pairs = fixtures::eviction_response_pairs(&docs);
    // Poison the classify response with a quote that is not in any span.
    let fabricated = json!({
        "docType": { "text": "Eviction notice",
            "citations": [{ "spanId": "D1-P1-S1", "quote": "the tribunal has already ruled on this matter" }] },
        "summary": { "text": "A summary.", "citations": [] }
    })
    .to_string();
    for pair in pairs.iter_mut() {
        if pair.0 == "classify" {
            pair.1 = fabricated.clone();
        }
    }
    let provider = MockProvider::from_pairs(pairs);

    let analysis = pipeline::analyze(
        &provider,
        &docs,
        Some("2026-09-10".to_string()),
        today(),
        Language::En,
        Vec::new(),
    )
    .await
    .expect("analysis succeeds");

    assert_eq!(
        analysis.paper_card.doc_type.status,
        VerificationStatus::NotConfirmed
    );
}

#[tokio::test]
async fn schema_repair_retry_recovers_from_bad_json() {
    let s = fixtures::eviction();
    // The first classify call returns junk; the repair retry falls back to the
    // valid default, so analysis still succeeds.
    s.provider.push_script("classify", "not json at all");
    let analysis = pipeline::analyze(
        &s.provider,
        &s.docs,
        s.anchor.clone(),
        today(),
        s.language,
        Vec::new(),
    )
    .await
    .expect("analysis recovers via repair retry");
    assert_eq!(
        analysis.paper_card.doc_type.status,
        VerificationStatus::Verified
    );
}

#[tokio::test]
async fn persistent_schema_failure_is_reported() {
    let s = fixtures::eviction();
    s.provider.push_script("classify", "still not json");
    s.provider.push_script("classify", "and neither is this");
    let result = pipeline::analyze(
        &s.provider,
        &s.docs,
        s.anchor.clone(),
        today(),
        s.language,
        Vec::new(),
    )
    .await;
    assert!(matches!(
        result,
        Err(olai_lib::error::AppError::SchemaViolation)
    ));
}
