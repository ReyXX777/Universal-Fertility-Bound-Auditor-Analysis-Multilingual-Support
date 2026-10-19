use crate::types::{LanguageCode, TokenizerName};
use serde::{Deserialize, Serialize};

/// Metrics for one (tokenizer, language) pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilityMetrics {
    pub tokenizer: String,
    pub language: String,
    pub total_tokens: u64,
    pub total_words: u64,
    pub total_chars: u64,
    pub total_graphemes: u64,
    pub single_token_words: u64,
    pub fertility: f64,
    pub chars_per_token: f64,
    pub graphemes_per_token: f64,
    pub single_token_retention_rate: f64,
    pub overhead_vs_reference: Option<f64>,
}

impl FertilityMetrics {
    pub fn new(tokenizer: &TokenizerName, language: &LanguageCode) -> RawAccumulator {
        RawAccumulator {
            tokenizer: tokenizer.clone(),
            language: language.clone(),
            total_tokens: 0,
            total_words: 0,
            total_chars: 0,
            total_graphemes: 0,
            single_token_words: 0,
        }
    }
}

/// Accumulator - pre-allocate and incremental
pub struct RawAccumulator {
    pub tokenizer: TokenizerName,
    pub language: LanguageCode,
    pub total_tokens: u64,
    pub total_words: u64,
    pub total_chars: u64,
    pub total_graphemes: u64,
    pub single_token_words: u64,
}

impl RawAccumulator {
    pub fn add_doc(&mut self, word_count: u64, token_count: u64, char_count: u64, grapheme_count: u64, single_token_word_count: u64) {
        self.total_words += word_count;
        self.total_tokens += token_count;
        self.total_chars += char_count;
        self.total_graphemes += grapheme_count;
        self.single_token_words += single_token_word_count;
    }

    pub fn finalize(self) -> FertilityMetrics {
        let fertility = if self.total_words > 0 {
            self.total_tokens as f64 / self.total_words as f64
        } else {
            0.0
        };
        let cpt = if self.total_tokens > 0 {
            self.total_chars as f64 / self.total_tokens as f64
        } else {
            0.0
        };
        let gpt = if self.total_tokens > 0 {
            self.total_graphemes as f64 / self.total_tokens as f64
        } else {
            0.0
        };
        let strr = if self.total_words > 0 {
            self.single_token_words as f64 / self.total_words as f64
        } else {
            0.0
        };

        FertilityMetrics {
            tokenizer: self.tokenizer.as_str().to_string(),
            language: self.language.as_str().to_string(),
            total_tokens: self.total_tokens,
            total_words: self.total_words,
            total_chars: self.total_chars,
            total_graphemes: self.total_graphemes,
            single_token_words: self.single_token_words,
            fertility,
            chars_per_token: cpt,
            graphemes_per_token: gpt,
            single_token_retention_rate: strr,
            overhead_vs_reference: None,
        }
    }
}
