use crate::error::{AnalyzerError, Result};
use crate::types::{LanguageCode, OverheadThreshold};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    pub reference_language: String,
    pub overhead_threshold: f64,
    pub languages: Vec<String>,
    pub tokenizers: Vec<String>,
}

impl AnalyzerConfig {
    pub fn validate(&self) -> Result<(LanguageCode, OverheadThreshold, Vec<LanguageCode>)> {
        let reference = LanguageCode::new(&self.reference_language)?;
        let threshold = OverheadThreshold::new(self.overhead_threshold)?;
        let mut langs = Vec::with_capacity(self.languages.len());
        for l in &self.languages {
            langs.push(LanguageCode::new(l)?);
        }
        if langs.is_empty() {
            return Err(AnalyzerError::Config("languages list empty".into()));
        }
        Ok((reference, threshold, langs))
    }
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            reference_language: "en".into(),
            overhead_threshold: 2.0,
            languages: vec!["en".into(), "hi".into(), "zh".into(), "ta".into(), "code".into()],
            tokenizers: vec!["whitespace_ideal".into(), "english_centric_bpe".into(), "char_level".into()],
        }
    }
}
