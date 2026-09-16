//! Detection and masking of personally identifiable information.
//!
//! PII is masked in previews, exports, and logs. Detection runs in a priority
//! order so overlapping matches (e.g. a 12-digit run) are labelled once, and
//! masks keep only the last few characters for user recognisability.

use crate::models::{PiiKind, PiiSpan};
use once_cell::sync::Lazy;
use regex::Regex;

/// A compiled detector: kind, pattern, and priority (lower wins on overlap).
struct Detector {
    kind: PiiKind,
    re: Regex,
}

static DETECTORS: Lazy<Vec<Detector>> = Lazy::new(|| {
    let specs: [(PiiKind, &str); 6] = [
        (
            PiiKind::Email,
            r"[A-Za-z0-9._%+\-]+@[A-Za-z0-9\-]+\.[A-Za-z]{2,}",
        ),
        (PiiKind::Upi, r"[A-Za-z0-9.\-_]{2,}@[A-Za-z]{2,}"),
        (PiiKind::Aadhaar, r"\b\d{4}\s\d{4}\s\d{4}\b|\b\d{12}\b"),
        (PiiKind::Pan, r"\b[A-Z]{5}\d{4}[A-Z]\b"),
        // Leading `\b` before the first mobile digit prevents matching a
        // 10-digit window inside a longer bank/Aadhaar run; internal `[\s-]?`
        // tolerates grouped formats like `98765 43210`.
        (
            PiiKind::Phone,
            r"(?:\+?91[\s\-]?)?\b[6-9]\d(?:[\s\-]?\d){8}\b",
        ),
        (PiiKind::BankAccount, r"\b\d{11,18}\b"),
    ];
    // `filter_map` + `ok()` avoids `expect`; these static patterns always compile.
    specs
        .into_iter()
        .filter_map(|(kind, p)| Regex::new(p).ok().map(|re| Detector { kind, re }))
        .collect()
});

/// Keep the last `keep` alphanumeric characters, mask the rest with `X`.
fn mask_keep_last(matched: &str, keep: usize) -> String {
    let digits: Vec<char> = matched.chars().filter(|c| c.is_alphanumeric()).collect();
    if digits.len() <= keep {
        return "X".repeat(digits.len().max(1));
    }
    let masked = "X".repeat(digits.len() - keep);
    let tail: String = digits[digits.len() - keep..].iter().collect();
    format!("{masked}{tail}")
}

/// Mask an email/UPI as `a***@domain`.
fn mask_handle(matched: &str) -> String {
    match matched.split_once('@') {
        Some((local, domain)) => {
            let first = local.chars().next().unwrap_or('x');
            format!("{first}***@{domain}")
        }
        None => "***".to_string(),
    }
}

/// Produce the masked replacement for a given kind and match.
fn mask_for(kind: PiiKind, matched: &str) -> String {
    match kind {
        PiiKind::Aadhaar => {
            let last4 = mask_keep_last(matched, 4);
            let tail = &last4[last4.len().saturating_sub(4)..];
            // Group as XXXX-XXXX-#### for readability.
            format!("XXXX-XXXX-{tail}")
        }
        PiiKind::Pan => mask_keep_last(matched, 4),
        PiiKind::Phone => mask_keep_last(matched, 4),
        PiiKind::BankAccount => mask_keep_last(matched, 4),
        PiiKind::Email | PiiKind::Upi => mask_handle(matched),
    }
}

/// Detect all PII occurrences, resolving overlaps by detector priority.
pub fn detect(text: &str) -> Vec<PiiSpan> {
    let mut candidates: Vec<(usize, PiiKind, usize, usize)> = Vec::new();
    for (priority, det) in DETECTORS.iter().enumerate() {
        for m in det.re.find_iter(text) {
            candidates.push((priority, det.kind, m.start(), m.end()));
        }
    }
    // Prefer higher priority (lower index); on ties prefer longer, then earlier.
    candidates.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then((b.3 - b.2).cmp(&(a.3 - a.2)))
            .then(a.2.cmp(&b.2))
    });

    let mut accepted: Vec<PiiSpan> = Vec::new();
    for (_, kind, start, end) in candidates {
        if accepted.iter().any(|s| start < s.end && end > s.start) {
            continue; // overlaps an already-accepted span
        }
        accepted.push(PiiSpan {
            kind,
            start,
            end,
            masked: mask_for(kind, &text[start..end]),
        });
    }
    accepted.sort_by_key(|s| s.start);
    accepted
}

/// Return `text` with every detected PII occurrence replaced by its mask.
pub fn mask(text: &str) -> String {
    let spans = detect(text);
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0usize;
    for span in spans {
        if span.start >= cursor {
            out.push_str(&text[cursor..span.start]);
            out.push_str(&span.masked);
            cursor = span.end;
        }
    }
    out.push_str(&text[cursor..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_aadhaar_keeping_last_four() {
        let out = mask("Aadhaar 1234 5678 9012 on file");
        assert!(out.contains("XXXX-XXXX-9012"), "got {out}");
        assert!(!out.contains("1234 5678 9012"));
    }

    #[test]
    fn masks_pan() {
        let spans = detect("PAN ABCDE1234F here");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].kind, PiiKind::Pan);
    }

    #[test]
    fn masks_email_and_upi_distinctly() {
        let out = mask("Write to jane.doe@example.com or pay 9876543210@ybl");
        assert!(out.contains("j***@example.com"), "got {out}");
        assert!(out.contains("@ybl"));
        assert!(!out.contains("jane.doe@example.com"));
    }

    #[test]
    fn masks_phone_last_four() {
        let out = mask("Call +91 98765 43210 today");
        assert!(out.contains("3210"), "got {out}");
        assert!(!out.contains("98765 43210"));
    }

    #[test]
    fn twelve_digit_run_is_aadhaar_not_bank() {
        let spans = detect("number 123456789012 end");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].kind, PiiKind::Aadhaar);
    }

    #[test]
    fn long_run_is_bank_account() {
        let spans = detect("acct 123456789012345 end");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].kind, PiiKind::BankAccount);
    }

    #[test]
    fn clean_text_is_unchanged() {
        let input = "The tenant must vacate within seven days.";
        assert_eq!(mask(input), input);
        assert!(detect(input).is_empty());
    }
}
