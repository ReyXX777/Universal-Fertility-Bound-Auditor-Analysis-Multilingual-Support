use crate::error::{AnalyzerError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Newtype: LanguageCode - encodes constraint: non-empty, ascii alphanumeric + _ + -, lowercased
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct LanguageCode(String);

impl LanguageCode {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self> {
        let raw = s.as_ref().trim();
        if raw.is_empty() {
            return Err(AnalyzerError::InvalidLanguageCode("empty".into()));
        }
        // allow en, hi, zh, ta, code, en-US etc
        if !raw.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' ) {
            return Err(AnalyzerError::InvalidLanguageCode(raw.to_string()));
        }
        Ok(Self(raw.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Zero-copy accessor
    pub fn as_slice(&self) -> &str {
        self.as_str()
    }

    pub fn uses_spaces(&self) -> bool {
        // heuristic: CJK languages don't use spaces
        !matches!(self.0.as_str(), "zh" | "ja" | "th" | "lo" | "my" | "km")
    }
}

impl fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Newtype: TokenizerName
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenizerName(String);

impl TokenizerName {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self> {
        let raw = s.as_ref().trim();
        if raw.is_empty() {
            return Err(AnalyzerError::InvalidTokenizerName("empty".into()));
        }
        Ok(Self(raw.to_string()))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for TokenizerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Counters as newtypes to prevent mixing
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WordCount(pub u64);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenCount(pub u64);
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharCount(pub u64);

impl From<u64> for WordCount { fn from(v: u64) -> Self { Self(v) } }
impl From<u64> for TokenCount { fn from(v: u64) -> Self { Self(v) } }

/// Threshold newtype
#[derive(Debug, Clone, Copy)]
pub struct OverheadThreshold(pub f64);

impl OverheadThreshold {
    pub fn new(v: f64) -> Result<Self> {
        if v < 1.0 || !v.is_finite() {
            return Err(AnalyzerError::Config(format!("threshold must be >=1.0, got {}", v)));
        }
        Ok(Self(v))
    }
    pub fn value(self) -> f64 { self.0 }
}
