//! The eviction-notice + rental-agreement demo scenario (fictional).

use super::{build_doc, cite, Scenario};
use crate::models::{DocRole, Document, Language};
use crate::providers::mock::MockProvider;
use serde_json::json;

/// Fictional eviction notice with the intended red flags: 7-day deadline,
/// full deposit forfeiture, maintenance to a personal UPI id, no itemisation.
const EVICTION_TEXT: &str = "EVICTION NOTICE\n\
To: Tenant, Flat 3B\n\
From: Landlord Mr Rao\n\
You must vacate the premises within 7 days of this notice.\n\
Your security deposit of Rs 60,000 is forfeited in full.\n\
Pay all pending maintenance charges immediately to UPI id landlord@okaxis.\n\
No itemised list of damages is provided.";

/// Fictional rental agreement contradicting the notice on three points.
const AGREEMENT_TEXT: &str = "RENTAL AGREEMENT\n\
Clause 6: The security deposit is refundable within 30 days, less itemised documented damages.\n\
Clause 9: Either party must give one month written notice to end the tenancy.\n\
Clause 12: Maintenance charges are included in the monthly rent.";

/// Build the two demo documents (notice + agreement).
pub fn eviction_docs() -> Vec<Document> {
    vec![
        build_doc(1, "eviction_notice.pdf", DocRole::Notice, EVICTION_TEXT).0,
        build_doc(
            2,
            "rental_agreement.pdf",
            DocRole::MyAgreement,
            AGREEMENT_TEXT,
        )
        .0,
    ]
}

/// Recorded `(key, json)` responses for every pipeline step, with citations
/// resolved against the given documents.
pub fn eviction_response_pairs(docs: &[Document]) -> Vec<(String, String)> {
    let classify = json!({
        "docType": { "text": "Eviction notice", "citations": [cite(docs, "EVICTION NOTICE", "EVICTION NOTICE")] },
        "parties": [
            { "role": "Landlord", "claim": { "text": "The landlord who issued the notice.", "citations": [cite(docs, "From: Landlord", "Landlord Mr Rao")] } },
            { "role": "Tenant", "claim": { "text": "The tenant receiving the notice.", "citations": [cite(docs, "To: Tenant", "Tenant")] } }
        ],
        "summary": { "text": "A notice asking the tenant to vacate within 7 days.", "citations": [cite(docs, "vacate the premises", "vacate the premises within 7 days")] }
    })
    .to_string();

    let extract = json!({
        "claims": [
            { "text": "The notice says the security deposit is forfeited.", "citations": [cite(docs, "security deposit", "security deposit of Rs 60,000 is forfeited")] }
        ],
        "deadlines": [
            { "label": "Vacate the premises", "unit": "days", "amount": 7,
              "claim": { "text": "Vacate within 7 days of the notice.", "citations": [cite(docs, "vacate the premises", "vacate the premises within 7 days")] } }
        ]
    })
    .to_string();

    let compare = json!({ "conflicts": [
        { "title": "Notice period is shorter than agreed",
          "notice": { "text": "The notice gives 7 days.", "citations": [cite(docs, "vacate the premises", "vacate the premises within 7 days")] },
          "agreement": { "text": "Clause 9 requires one month.", "citations": [cite(docs, "one month written notice", "one month written notice")] },
          "explanation": "The notice gives less time than the agreement's clause 9.", "severity": "high" },
        { "title": "Deposit forfeiture differs from the agreement",
          "notice": { "text": "The notice forfeits the whole deposit.", "citations": [cite(docs, "security deposit", "security deposit of Rs 60,000 is forfeited")] },
          "agreement": { "text": "Clause 6 refunds within 30 days less itemised damages.", "citations": [cite(docs, "refundable within 30 days", "refundable within 30 days, less itemised documented damages")] },
          "explanation": "The agreement allows a refund less itemised damages, not full forfeiture.", "severity": "high" },
        { "title": "Maintenance charge differs from the agreement",
          "notice": { "text": "The notice demands maintenance now.", "citations": [cite(docs, "maintenance charges immediately", "maintenance charges immediately")] },
          "agreement": { "text": "Clause 12 includes maintenance in rent.", "citations": [cite(docs, "included in the monthly rent", "included in the monthly rent")] },
          "explanation": "The agreement says maintenance is included in the rent.", "severity": "medium" }
    ] })
    .to_string();

    let actions = json!({ "actions": [
        { "group": "protect", "text": "Keep the original notice and a copy of your agreement in a safe place.", "citations": [] },
        { "group": "gather", "text": "Note the deposit amount and collect any payment receipts.", "citations": [cite(docs, "security deposit", "security deposit of Rs 60,000 is forfeited")] },
        { "group": "respond", "text": "You may reply in writing asking for an itemised list of damages.", "citations": [cite(docs, "No itemised list", "No itemised list of damages is provided")] },
        { "group": "getHelp", "text": "Consider contacting a free legal-aid service to review the notice.", "citations": [] }
    ] })
    .to_string();

    let qa = json!({ "status": "answered",
        "text": "Your agreement's Clause 9 asks for one month's written notice.",
        "citations": [cite(docs, "one month written notice", "one month written notice")] })
    .to_string();

    let reply = json!({
        "text": "Dear Sir, I acknowledge receipt of your notice. Our agreement asks for one month's written notice and allows the deposit to be refunded less any itemised, documented damages. Please share an itemised list of damages and reconsider the timeline. Thank you.",
        "footnotes": [
            cite(docs, "one month written notice", "one month written notice"),
            cite(docs, "refundable within 30 days", "refundable within 30 days, less itemised documented damages")
        ]
    })
    .to_string();

    vec![
        ("classify".into(), classify),
        ("extract:notice".into(), extract),
        ("compare".into(), compare),
        ("actions".into(), actions),
        ("qa".into(), qa),
        ("reply".into(), reply),
    ]
}

/// The full eviction demo scenario.
pub fn eviction() -> Scenario {
    let (notice, mut findings) =
        build_doc(1, "eviction_notice.pdf", DocRole::Notice, EVICTION_TEXT);
    let (agreement, more) = build_doc(
        2,
        "rental_agreement.pdf",
        DocRole::MyAgreement,
        AGREEMENT_TEXT,
    );
    findings.extend(more);
    let docs = vec![notice, agreement];
    let provider = MockProvider::from_pairs(eviction_response_pairs(&docs));
    Scenario {
        docs,
        findings,
        provider,
        anchor: Some("2026-09-10".to_string()),
        language: Language::En,
    }
}
