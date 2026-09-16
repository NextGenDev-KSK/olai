//! Deadline arithmetic. All date math is done here with `chrono` — never by the
//! model. Every result carries a human-readable explanation of how it was
//! derived, and a missing anchor date is reported (never guessed).

use crate::models::DeadlineCalc;
use chrono::{Datelike, Months, NaiveDate};

/// An interval expressed by a notice or agreement clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    /// A number of days.
    Days(i64),
    /// A number of calendar months (month-end clamped).
    Months(i64),
}

/// The outcome of a deadline computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Computed {
    /// Resolved date, if the anchor was known.
    pub date: Option<NaiveDate>,
    /// Days remaining relative to "today" (may be negative), if resolvable.
    pub days_remaining: Option<i64>,
    /// Explanation of the arithmetic and whether an anchor is still needed.
    pub calc: DeadlineCalc,
}

/// Parse an anchor date. Accepts ISO `YYYY-MM-DD`, or `DD/MM/YYYY` /
/// `DD-MM-YYYY`. Ambiguous numeric dates are read as **day/month** per the
/// Indian convention; the assumption is surfaced in the explanation.
pub fn parse_anchor(input: &str) -> Option<NaiveDate> {
    let s = input.trim();
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(d);
    }
    for fmt in ["%d/%m/%Y", "%d-%m-%Y"] {
        if let Ok(d) = NaiveDate::parse_from_str(s, fmt) {
            return Some(d);
        }
    }
    None
}

/// Whether an anchor string is numeric-ambiguous (used to note the DD/MM rule).
fn is_numeric_format(input: &str) -> bool {
    let s = input.trim();
    NaiveDate::parse_from_str(s, "%Y-%m-%d").is_err()
        && (NaiveDate::parse_from_str(s, "%d/%m/%Y").is_ok()
            || NaiveDate::parse_from_str(s, "%d-%m-%Y").is_ok())
}

/// Add days, returning `None` on overflow.
pub fn add_days(date: NaiveDate, n: i64) -> Option<NaiveDate> {
    date.checked_add_signed(chrono::Duration::try_days(n)?)
}

/// Add months with month-end clamping (e.g. Jan 31 + 1 month = Feb 28/29).
pub fn add_months(date: NaiveDate, n: i64) -> Option<NaiveDate> {
    if n >= 0 {
        date.checked_add_months(Months::new(n as u32))
    } else {
        date.checked_sub_months(Months::new((-n) as u32))
    }
}

/// Compute a deadline from an optional anchor and an interval.
pub fn compute(
    anchor: Option<(NaiveDate, String)>,
    interval: Interval,
    today: NaiveDate,
) -> Computed {
    let Some((anchor_date, raw)) = anchor else {
        return Computed {
            date: None,
            days_remaining: None,
            calc: DeadlineCalc {
                anchor: None,
                steps: vec![missing_anchor_step(interval)],
                needs_anchor: true,
            },
        };
    };

    let (resolved, mut steps) = apply_interval(anchor_date, interval);
    if is_numeric_format(&raw) {
        steps.push("Numeric date read as day/month (DD/MM).".to_string());
    }
    let days_remaining = resolved.map(|d| (d - today).num_days());

    Computed {
        date: resolved,
        days_remaining,
        calc: DeadlineCalc {
            anchor: Some(anchor_date.format("%Y-%m-%d").to_string()),
            steps,
            needs_anchor: false,
        },
    }
}

/// Apply the interval and describe the step.
fn apply_interval(anchor: NaiveDate, interval: Interval) -> (Option<NaiveDate>, Vec<String>) {
    match interval {
        Interval::Days(n) => {
            let out = add_days(anchor, n);
            let step = match out {
                Some(d) => format!("{anchor} + {n} days = {d}"),
                None => format!("{anchor} + {n} days = (out of range)"),
            };
            (out, vec![step])
        }
        Interval::Months(n) => {
            let out = add_months(anchor, n);
            let clamp = out
                .filter(|d| d.day() < anchor.day())
                .map(|_| " (clamped to month end)")
                .unwrap_or("");
            let step = match out {
                Some(d) => format!("{anchor} + {n} months = {d}{clamp}"),
                None => format!("{anchor} + {n} months = (out of range)"),
            };
            (out, vec![step])
        }
    }
}

/// Explanation used when there is no anchor date.
fn missing_anchor_step(interval: Interval) -> String {
    match interval {
        Interval::Days(n) => format!("Needs the start date, then + {n} days."),
        Interval::Months(n) => format!("Needs the start date, then + {n} months."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn adds_days_with_explanation() {
        let out = compute(
            Some((d("2026-09-10"), "2026-09-10".into())),
            Interval::Days(7),
            d("2026-09-13"),
        );
        assert_eq!(out.date, Some(d("2026-09-17")));
        assert_eq!(out.days_remaining, Some(4));
        assert!(out.calc.steps[0].contains("+ 7 days = 2026-09-17"));
        assert!(!out.calc.needs_anchor);
    }

    #[test]
    fn adds_one_month() {
        let out = compute(
            Some((d("2026-03-15"), "2026-03-15".into())),
            Interval::Months(1),
            d("2026-03-15"),
        );
        assert_eq!(out.date, Some(d("2026-04-15")));
    }

    #[test]
    fn month_end_is_clamped() {
        let out = compute(
            Some((d("2026-01-31"), "2026-01-31".into())),
            Interval::Months(1),
            d("2026-01-31"),
        );
        assert_eq!(out.date, Some(d("2026-02-28")));
        assert!(out.calc.steps[0].contains("clamped to month end"));
    }

    #[test]
    fn missing_anchor_is_reported_not_guessed() {
        let out = compute(None, Interval::Days(7), d("2026-09-13"));
        assert!(out.calc.needs_anchor);
        assert_eq!(out.date, None);
        assert_eq!(out.days_remaining, None);
    }

    #[test]
    fn parses_iso_and_ddmm() {
        assert_eq!(parse_anchor("2026-06-05"), Some(d("2026-06-05")));
        // 05/06/2026 must be 5 June (DD/MM), not 6 May.
        assert_eq!(parse_anchor("05/06/2026"), Some(d("2026-06-05")));
        assert_eq!(parse_anchor("not a date"), None);
    }

    #[test]
    fn ddmm_assumption_is_surfaced() {
        let out = compute(
            Some((d("2026-06-05"), "05/06/2026".into())),
            Interval::Days(3),
            d("2026-06-05"),
        );
        assert!(out.calc.steps.iter().any(|s| s.contains("DD/MM")));
    }

    #[test]
    fn negative_days_remaining_when_overdue() {
        let out = compute(
            Some((d("2026-09-01"), "2026-09-01".into())),
            Interval::Days(7),
            d("2026-09-20"),
        );
        assert_eq!(out.days_remaining, Some(-12));
    }
}
