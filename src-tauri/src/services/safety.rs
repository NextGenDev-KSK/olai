//! Safety policy: help-tier selection with forced escalation, scam/court
//! detection, strategy-question detection, and the verdict-word output filter.

use crate::models::{HelpTier, TierDecision, TierReason};
use crate::services::pii;
use once_cell::sync::Lazy;
use regex::Regex;

/// Deadlines at or within this many days force the "needs a lawyer" tier.
pub const SHORT_DEADLINE_DAYS: i64 = 7;

/// The signals that can force escalation to [`HelpTier::NeedsLawyer`].
#[derive(Debug, Clone, Default)]
pub struct Triggers {
    /// A court or police document was detected.
    pub court_or_police: bool,
    /// The soonest deadline's days-remaining, if any.
    pub min_days_remaining: Option<i64>,
    /// Scam signals were detected.
    pub scam: bool,
    /// The user asked a strategy/outcome question.
    pub strategy: bool,
}

/// Decide the help tier, escalating to `NeedsLawyer` on any trigger.
pub fn decide_tier(base: HelpTier, triggers: &Triggers) -> TierDecision {
    let mut reasons = Vec::new();
    if triggers.court_or_police {
        reasons.push(TierReason::CourtOrPolice);
    }
    if triggers
        .min_days_remaining
        .is_some_and(|d| d <= SHORT_DEADLINE_DAYS)
    {
        reasons.push(TierReason::ShortDeadline);
    }
    if triggers.scam {
        reasons.push(TierReason::ScamSignal);
    }
    if triggers.strategy {
        reasons.push(TierReason::StrategyQuestion);
    }

    let tier = if reasons.is_empty() {
        reasons.push(TierReason::Baseline);
        base
    } else {
        base.max(HelpTier::NeedsLawyer)
    };
    TierDecision { tier, reasons }
}

static COURT_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(court|summons|magistrate|tribunal|warrant|hearing|police|f\.?i\.?r\.?|first information report|petition|litigation|subpoena)\b",
    )
    .ok()
});

/// Detect court/police document signals in text.
pub fn detect_court_or_police(text: &str) -> bool {
    COURT_RE.as_ref().is_some_and(|re| re.is_match(text))
}

static SCAM_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(gift card|google play|otp|processing fee|arrest|blocked|penalty of|wire the|western union|immediately (pay|transfer))\b",
    )
    .ok()
});

/// Detect scam signals: a personal UPI paired with a payment demand, or common
/// fraud phrases.
pub fn detect_scam_signals(text: &str) -> bool {
    let has_upi = pii::detect(text)
        .iter()
        .any(|s| matches!(s.kind, crate::models::PiiKind::Upi));
    let payment_words = Regex::new(r"(?i)\b(pay|transfer|remit|maintenance|deposit|fee|amount)\b")
        .ok()
        .is_some_and(|re| re.is_match(text));
    let phrase_hit = SCAM_RE.as_ref().is_some_and(|re| re.is_match(text));
    (has_upi && payment_words) || phrase_hit
}

static STRATEGY_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(
        r"(?i)(will i (win|lose)|should i (sign|sue|pay|fight)|what are my chances|how (do|can) i win|best strategy|guarantee|what will happen|predict|my odds|should i go to court)",
    )
    .ok()
});

/// Detect a question that asks for legal strategy or an outcome prediction.
pub fn is_strategy_question(question: &str) -> bool {
    STRATEGY_RE.as_ref().is_some_and(|re| re.is_match(question))
}

static VERDICT_RE: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(genuine|enforceable|unenforceable|legally binding|legally valid|you will (win|lose)|guaranteed to win|is (valid|invalid|legal|illegal))\b",
    )
    .ok()
});

/// Return the first blocked verdict phrase in model output, if any.
pub fn find_verdict_word(text: &str) -> Option<String> {
    VERDICT_RE
        .as_ref()
        .and_then(|re| re.find(text))
        .map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_tier_is_kept_without_triggers() {
        let d = decide_tier(HelpTier::GeneralGuidance, &Triggers::default());
        assert_eq!(d.tier, HelpTier::GeneralGuidance);
        assert_eq!(d.reasons, vec![TierReason::Baseline]);
    }

    #[test]
    fn short_deadline_forces_lawyer() {
        let t = Triggers {
            min_days_remaining: Some(5),
            ..Default::default()
        };
        let d = decide_tier(HelpTier::Information, &t);
        assert_eq!(d.tier, HelpTier::NeedsLawyer);
        assert!(d.reasons.contains(&TierReason::ShortDeadline));
    }

    #[test]
    fn overdue_deadline_also_escalates() {
        let t = Triggers {
            min_days_remaining: Some(-3),
            ..Default::default()
        };
        assert_eq!(
            decide_tier(HelpTier::Information, &t).tier,
            HelpTier::NeedsLawyer
        );
    }

    #[test]
    fn court_document_forces_lawyer() {
        let t = Triggers {
            court_or_police: true,
            ..Default::default()
        };
        assert_eq!(
            decide_tier(HelpTier::Information, &t).tier,
            HelpTier::NeedsLawyer
        );
    }

    #[test]
    fn detects_court_and_scam_and_strategy() {
        assert!(detect_court_or_police(
            "You are summoned to appear before the magistrate."
        ));
        assert!(detect_scam_signals(
            "Pay maintenance to name@ybl immediately"
        ));
        assert!(detect_scam_signals("Share the OTP to avoid arrest"));
        assert!(is_strategy_question("Will I win if I go to court?"));
        assert!(!is_strategy_question("What is the notice period?"));
    }

    #[test]
    fn verdict_filter_blocks_banned_words() {
        assert!(find_verdict_word("This notice is valid and enforceable").is_some());
        assert!(find_verdict_word("You will win this case").is_some());
        assert!(find_verdict_word("The notice sets a seven day deadline.").is_none());
    }
}
