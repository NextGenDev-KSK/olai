//! Splits page text into stable, quotable spans with `D{doc}-P{page}-S{n}` ids.
//!
//! Segmentation is deterministic: the same input always yields the same span
//! ids, which is what makes citations stable and testable.

use crate::models::{DocRole, Document, Page, SourceKind, Span};

/// Characters that terminate a sentence/segment across our supported scripts.
/// Includes the Devanagari/Tamil danda (`।`, U+0964) and double danda.
const TERMINATORS: [char; 6] = ['.', '!', '?', ';', '\u{0964}', '\u{0965}'];

/// Build a citation id in the canonical `D{doc}-P{page}-S{n}` format.
pub fn span_id(doc_index: u32, page: u32, seq: u32) -> String {
    format!("D{doc_index}-P{page}-S{seq}")
}

/// Segment a single page's text into spans. `seq` starts at 1 for each page.
pub fn segment_page(doc_index: u32, page: u32, text: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut seq: u32 = 0;
    for line in text.split('\n') {
        for raw in split_sentences(line) {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                continue;
            }
            seq += 1;
            spans.push(Span {
                id: span_id(doc_index, page, seq),
                doc_index,
                page,
                text: trimmed.to_string(),
                flags: Vec::new(),
            });
        }
    }
    spans
}

/// Split a single line into sentence-like fragments, keeping terminators.
fn split_sentences(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    for ch in line.chars() {
        current.push(ch);
        if TERMINATORS.contains(&ch) {
            out.push(std::mem::take(&mut current));
        }
    }
    if !current.trim().is_empty() {
        out.push(current);
    }
    out
}

/// Assemble a full [`Document`] from ordered pages.
pub fn segment_document(
    index: u32,
    file_name: String,
    role: DocRole,
    source: SourceKind,
    hash: String,
    pages: &[Page],
) -> Document {
    let mut spans = Vec::new();
    for page in pages {
        spans.extend(segment_page(index, page.number, &page.text));
    }
    Document {
        index,
        file_name,
        role,
        source,
        hash,
        page_count: pages.len() as u32,
        spans,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_id_uses_canonical_format() {
        assert_eq!(span_id(1, 2, 3), "D1-P2-S3");
    }

    #[test]
    fn segments_sentences_and_numbers_them_per_page() {
        let spans = segment_page(1, 1, "You must vacate. Deposit is forfeited.");
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].id, "D1-P1-S1");
        assert_eq!(spans[0].text, "You must vacate.");
        assert_eq!(spans[1].id, "D1-P1-S2");
        assert_eq!(spans[1].text, "Deposit is forfeited.");
    }

    #[test]
    fn newlines_split_segments() {
        let spans = segment_page(2, 5, "Clause 9\nOne month notice");
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].id, "D2-P5-S1");
        assert_eq!(spans[1].text, "One month notice");
    }

    #[test]
    fn empty_and_whitespace_lines_are_skipped() {
        let spans = segment_page(1, 1, "  \n\nHello world.\n   ");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].text, "Hello world.");
    }

    #[test]
    fn handles_devanagari_danda() {
        let spans = segment_page(1, 1, "यह एक वाक्य है। दूसरा वाक्य।");
        assert_eq!(spans.len(), 2);
    }

    #[test]
    fn document_numbers_pages_and_counts() {
        let pages = vec![
            Page {
                number: 1,
                text: "First. Second.".into(),
            },
            Page {
                number: 2,
                text: "Third.".into(),
            },
        ];
        let doc = segment_document(
            1,
            "n.pdf".into(),
            DocRole::Notice,
            SourceKind::PdfText,
            "h".into(),
            &pages,
        );
        assert_eq!(doc.page_count, 2);
        assert_eq!(doc.spans.len(), 3);
        assert_eq!(doc.spans[2].id, "D1-P2-S1");
    }
}
