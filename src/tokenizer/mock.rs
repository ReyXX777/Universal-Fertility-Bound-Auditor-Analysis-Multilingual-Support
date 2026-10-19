use crate::types::TokenizerName;
use crate::tokenizer::Tokenizer;
use crate::error::Result;

/// Ideal tokenizer: 1 token per word - fertility ≈ 1
pub struct WhitespaceIdealTokenizer {
    name: TokenizerName,
}

impl WhitespaceIdealTokenizer {
    pub fn new() -> Self {
        Self {
            name: TokenizerName::new("whitespace_ideal").unwrap_or_else(|_| TokenizerName::new("ideal").unwrap()),
        }
    }
}

impl Tokenizer for WhitespaceIdealTokenizer {
    fn name(&self) -> &TokenizerName { &self.name }

    fn count_tokens(&self, text: &str) -> usize {
        // Lean on iterators: lazy evaluation
        text.split_whitespace().count()
    }

    fn count_tokens_for_word(&self, _word: &str) -> usize {
        1
    }
}

/// Char-level tokenizer: each char = token -> high fertility, low cpt
pub struct CharTokenizer {
    name: TokenizerName,
}

impl CharTokenizer {
    pub fn new() -> Self {
        Self {
            name: TokenizerName::new("char_level").unwrap_or_else(|_| TokenizerName::new("char").unwrap()),
        }
    }
}

impl Tokenizer for CharTokenizer {
    fn name(&self) -> &TokenizerName { &self.name }

    fn count_tokens(&self, text: &str) -> usize {
        // UTF-8 reality: count chars, not bytes
        text.chars().filter(|c| !c.is_whitespace()).count()
    }

    fn count_tokens_for_word(&self, word: &str) -> usize {
        word.chars().count().max(1)
    }

    fn encode_debug(&self, text: &str) -> Vec<String> {
        text.chars().filter(|c| !c.is_whitespace()).map(|c| c.to_string()).collect()
    }
}
