//! Bundled SYNTHETIC documents and recorded model responses for DEMO MODE and
//! integration tests. All names, figures, and identifiers are fictional.
//!
//! Recorded responses are generated deterministically from the synthetic
//! documents (citations are located by phrase), so every quote verifies exactly
//! against the segmented spans regardless of span numbering.

mod eviction;
mod scam;

pub use eviction::{eviction, eviction_docs, eviction_response_pairs};
pub use scam::scam;

use crate::models::{DocRole, Document, InjectionFinding, Language, Page, SourceKind};
use crate::providers::mock::MockProvider;
use crate::services::{injection, segmentation};
use serde_json::{json, Value};

/// A ready-to-run demo scenario.
pub struct Scenario {
    /// The segmented synthetic documents.
    pub docs: Vec<Document>,
    /// Injection/hidden-text findings detected during ingestion.
    pub findings: Vec<InjectionFinding>,
    /// A mock provider preloaded with recorded responses.
    pub provider: MockProvider,
    /// The optional received-date anchor.
    pub anchor: Option<String>,
    /// The scenario's default language.
    pub language: Language,
}

/// Build one synthetic single-page document and scan it for injection/hidden text.
pub(crate) fn build_doc(
    index: u32,
    file_name: &str,
    role: DocRole,
    text: &str,
) -> (Document, Vec<InjectionFinding>) {
    let pages = vec![Page {
        number: 1,
        text: text.to_string(),
    }];
    let mut doc = segmentation::segment_document(
        index,
        file_name.to_string(),
        role,
        SourceKind::PdfText,
        format!("demo-hash-{index}"),
        &pages,
    );
    let findings = injection::scan_spans(&mut doc.spans);
    (doc, findings)
}

/// Span id of the first span containing `needle` (case-insensitive).
pub(crate) fn span_id(docs: &[Document], needle: &str) -> String {
    let needle = needle.to_lowercase();
    for doc in docs {
        for span in &doc.spans {
            if span.text.to_lowercase().contains(&needle) {
                return span.id.clone();
            }
        }
    }
    String::new()
}

/// Build a `{spanId, quote}` citation value locating the span by phrase.
pub(crate) fn cite(docs: &[Document], needle: &str, quote: &str) -> Value {
    json!({ "spanId": span_id(docs, needle), "quote": quote })
}
