//! Session-level value types: languages and the help disclaimer.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Supported UI and content languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// English (default).
    #[default]
    En,
    /// Tamil (தமிழ்).
    Ta,
    /// Hindi (हिन्दी).
    Hi,
}

impl Language {
    /// BCP-47 language tag for the `lang` attribute.
    pub fn bcp47(self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Ta => "ta",
            Language::Hi => "hi",
        }
    }
}

/// A newtype for opaque, validated identifiers (session, doc references).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/lib/bindings/")]
pub struct SessionId(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcp47_tags_are_correct() {
        assert_eq!(Language::En.bcp47(), "en");
        assert_eq!(Language::Ta.bcp47(), "ta");
        assert_eq!(Language::Hi.bcp47(), "hi");
    }

    #[test]
    fn default_language_is_english() {
        assert_eq!(Language::default(), Language::En);
    }
}
