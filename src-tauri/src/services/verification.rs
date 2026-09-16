//! Citation verification: confirms every model quote against stored span text.
//!
//! A quote must be at most 25 words and match its span with a normalized fuzzy
//! similarity of at least [`SIMILARITY_THRESHOLD`]. Unconfirmed claims are
//! marked [`VerificationStatus::NotConfirmed`] so the UI can grey them out.

use crate::models::{Citation, Claim, VerificationStatus};
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

/// Minimum normalized similarity for a citation to be confirmed.
pub const SIMILARITY_THRESHOLD: f32 = 0.9;

/// Maximum allowed quote length in words.
pub const MAX_QUOTE_WORDS: usize = 25;

/// Normalize text for robust comparison: NFKC, lowercase, alphanumeric +
/// single spaces only. This tolerates OCR punctuation/spacing noise.
pub fn normalize(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_was_space = true; // trims leading space
    for ch in input.nfkc() {
        if ch.is_alphanumeric() {
            for lower in ch.to_lowercase() {
                out.push(lower);
            }
            last_was_space = false;
        } else if ch.is_whitespace() && !last_was_space {
            out.push(' ');
            last_was_space = true;
        }
        // all other punctuation is dropped
    }
    if out.ends_with(' ') {
        out.pop();
    }
    out
}

/// Count words in the raw (un-normalized) quote.
pub fn word_count(quote: &str) -> usize {
    quote.split_whitespace().filter(|w| !w.is_empty()).count()
}

/// Similarity in `[0, 1]` of `needle` against the best-matching substring of
/// `haystack`, using normalized text and a fuzzy (Levenshtein) substring match.
pub fn similarity(needle: &str, haystack: &str) -> f32 {
    let q: Vec<char> = normalize(needle).chars().collect();
    let t: Vec<char> = normalize(haystack).chars().collect();
    if q.is_empty() {
        return 0.0;
    }
    let n = q.len();
    let m = t.len();
    // Row 0 is all zeros: the needle may start matching anywhere in haystack.
    let mut prev: Vec<usize> = vec![0; m + 1];
    for (i, qc) in q.iter().enumerate() {
        let mut curr = vec![0usize; m + 1];
        curr[0] = i + 1; // deleting i+1 needle chars against an empty prefix
        for j in 1..=m {
            let cost = if *qc == t[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        prev = curr;
    }
    let best = prev.iter().copied().min().unwrap_or(n);
    (1.0 - best as f32 / n as f32).max(0.0)
}

/// Verify a single quote against a span's stored text.
///
/// Returns `(similarity, confirmed)`. A quote longer than [`MAX_QUOTE_WORDS`]
/// is never confirmed regardless of similarity.
pub fn verify_quote(span_text: &str, quote: &str) -> (f32, bool) {
    if word_count(quote) > MAX_QUOTE_WORDS {
        return (0.0, false);
    }
    let sim = similarity(quote, span_text);
    (sim, sim >= SIMILARITY_THRESHOLD)
}

/// Verify every citation on a claim against the provided span-text lookup and
/// return a claim with per-citation results and an overall status.
///
/// A claim is [`VerificationStatus::Verified`] only when it has at least one
/// citation and *all* citations are confirmed.
pub fn verify_claim(mut claim: Claim, span_text: &HashMap<String, String>) -> Claim {
    if claim.citations.is_empty() {
        claim.status = VerificationStatus::NotConfirmed;
        return claim;
    }
    let mut all_ok = true;
    let checked: Vec<Citation> = claim
        .citations
        .into_iter()
        .map(|mut c| {
            match span_text.get(&c.span_id) {
                Some(text) => {
                    let (sim, ok) = verify_quote(text, &c.quote);
                    c.similarity = sim;
                    c.confirmed = ok;
                }
                None => {
                    c.similarity = 0.0;
                    c.confirmed = false;
                }
            }
            all_ok &= c.confirmed;
            c
        })
        .collect();
    claim.citations = checked;
    claim.status = if all_ok {
        VerificationStatus::Verified
    } else {
        VerificationStatus::NotConfirmed
    };
    claim
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cite(span_id: &str, quote: &str) -> Citation {
        Citation {
            span_id: span_id.to_string(),
            quote: quote.to_string(),
            page: 1,
            similarity: 0.0,
            confirmed: false,
        }
    }

    #[test]
    fn exact_quote_is_confirmed() {
        let (sim, ok) = verify_quote(
            "You must vacate within seven days.",
            "vacate within seven days",
        );
        assert!(sim > 0.99, "sim was {sim}");
        assert!(ok);
    }

    #[test]
    fn ocr_noisy_quote_still_confirmed() {
        // A realistic single-character OCR misread (one "0" read as "O") plus a
        // dropped period: small edit distance, still >= 0.9.
        let span = "Deposit of Rs 60,000 is forfeited.";
        let (sim, ok) = verify_quote(span, "Deposit of Rs 6O,000 is forfeited");
        assert!(sim >= 0.9, "sim was {sim}");
        assert!(ok);
    }

    #[test]
    fn fabricated_quote_is_rejected() {
        let span = "The tenant must vacate the premises within seven days.";
        let (sim, ok) = verify_quote(span, "You will definitely win this case in court");
        assert!(sim < 0.9, "sim was {sim}");
        assert!(!ok);
    }

    #[test]
    fn over_length_quote_is_never_confirmed() {
        let long = (0..30)
            .map(|i| format!("word{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let span = long.clone();
        let (sim, ok) = verify_quote(&span, &long);
        assert_eq!(sim, 0.0);
        assert!(!ok);
    }

    #[test]
    fn normalize_collapses_and_lowercases() {
        assert_eq!(normalize("  Rs.  60,000!! "), "rs 60000");
    }

    #[test]
    fn claim_with_all_confirmed_is_verified() {
        let mut map = HashMap::new();
        map.insert(
            "D1-P1-S1".to_string(),
            "Vacate within seven days.".to_string(),
        );
        let claim = Claim {
            id: "c1".into(),
            text: "You have seven days.".into(),
            citations: vec![cite("D1-P1-S1", "Vacate within seven days")],
            status: VerificationStatus::NotConfirmed,
        };
        let out = verify_claim(claim, &map);
        assert_eq!(out.status, VerificationStatus::Verified);
        assert!(out.citations[0].confirmed);
    }

    #[test]
    fn claim_with_missing_span_is_not_confirmed() {
        let map = HashMap::new();
        let claim = Claim {
            id: "c1".into(),
            text: "x".into(),
            citations: vec![cite("D9-P9-S9", "anything")],
            status: VerificationStatus::Verified,
        };
        let out = verify_claim(claim, &map);
        assert_eq!(out.status, VerificationStatus::NotConfirmed);
        assert_eq!(out.citations[0].similarity, 0.0);
    }

    #[test]
    fn claim_without_citations_is_not_confirmed() {
        let map = HashMap::new();
        let claim = Claim {
            id: "c1".into(),
            text: "unsupported".into(),
            citations: vec![],
            status: VerificationStatus::Verified,
        };
        assert_eq!(
            verify_claim(claim, &map).status,
            VerificationStatus::NotConfirmed
        );
    }
}
