//! Document, page, and span types. Spans carry stable citation ids.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// The role a user assigns to an uploaded document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum DocRole {
    /// The legal notice the user received.
    Notice,
    /// The user's own agreement/contract.
    MyAgreement,
    /// Any other supporting document.
    Other,
}

/// How a document's text was obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum SourceKind {
    /// Embedded text extracted from a PDF.
    PdfText,
    /// Text produced by vision-model OCR of an image.
    ImageOcr,
}

/// A safety flag attached to a span (e.g. prompt-injection or hidden text).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub enum SpanFlag {
    /// Contains an instruction-like phrase aimed at the model.
    InjectionPhrase,
    /// Contains zero-width or otherwise hidden characters.
    HiddenText,
}

/// A contiguous, quotable unit of document text with a stable id.
///
/// The id format is `D{doc}-P{page}-S{n}` (see `services::segmentation`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Span {
    /// Stable citation id, e.g. `D1-P2-S3`.
    pub id: String,
    /// 1-based document index.
    pub doc_index: u32,
    /// 1-based page number.
    pub page: u32,
    /// Verbatim text of the span.
    pub text: String,
    /// Any safety flags detected on this span.
    pub flags: Vec<SpanFlag>,
}

/// A single page's worth of extracted text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Page {
    /// 1-based page number.
    pub number: u32,
    /// Full page text (unsegmented), used only for extraction/segmentation.
    pub text: String,
}

/// An ingested document, held only in memory for the session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// 1-based document index used in span ids.
    pub index: u32,
    /// Original file name (basename only; never a full path).
    pub file_name: String,
    /// User-assigned role.
    pub role: DocRole,
    /// How the text was obtained.
    pub source: SourceKind,
    /// SHA-256 of the raw bytes, used as a cache key.
    pub hash: String,
    /// Segmented spans, in reading order.
    pub spans: Vec<Span>,
    /// Page count.
    pub page_count: u32,
}

impl Document {
    /// Look up a span by its citation id.
    pub fn span(&self, id: &str) -> Option<&Span> {
        self.spans.iter().find(|s| s.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(id: &str) -> Span {
        Span {
            id: id.to_string(),
            doc_index: 1,
            page: 1,
            text: "hello".into(),
            flags: vec![],
        }
    }

    #[test]
    fn span_lookup_finds_and_misses() {
        let doc = Document {
            index: 1,
            file_name: "notice.pdf".into(),
            role: DocRole::Notice,
            source: SourceKind::PdfText,
            hash: "abc".into(),
            spans: vec![span("D1-P1-S1"), span("D1-P1-S2")],
            page_count: 1,
        };
        assert!(doc.span("D1-P1-S2").is_some());
        assert!(doc.span("D1-P1-S9").is_none());
    }
}
