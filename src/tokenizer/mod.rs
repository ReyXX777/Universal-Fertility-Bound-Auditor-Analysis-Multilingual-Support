pub mod mock;
pub mod simple_bpe;

use crate::types::TokenizerName;

pub trait Tokenizer: Send + Sync {
    fn name(&self) -> &TokenizerName;

    /// Returns number of tokens for full text - efficient, no allocation of token strings
    /// Uses iterators and zero-copy where possible
    fn count_tokens(&self, text: &str) -> usize;

    /// Returns whether a single word would be single token (for STRR)
    fn is_single_token(&self, word: &str) -> bool {
        self.count_tokens_for_word(word) == 1
    }

    fn count_tokens_for_word(&self, word: &str) -> usize;

    /// Optional: return token strings for debugging (allocates)
    fn encode_debug(&self, text: &str) -> Vec<String> {
        // default: produce dummy tokens based on count
        let n = self.count_tokens(text);
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(format!("tok_{}", i));
        }
        v
    }
}

pub use mock::{CharTokenizer, WhitespaceIdealTokenizer};
pub use simple_bpe::SimpleBpeTokenizer;

/// Factory for demo tokenizers
pub fn create_tokenizers() -> Vec<Box<dyn Tokenizer>> {
    let mut tokenizers: Vec<Box<dyn Tokenizer>> = Vec::with_capacity(3);
    tokenizers.push(Box::new(WhitespaceIdealTokenizer::new()));
    tokenizers.push(Box::new(CharTokenizer::new()));
    tokenizers.push(Box::new(SimpleBpeTokenizer::english_centric()));
    tokenizers
}
