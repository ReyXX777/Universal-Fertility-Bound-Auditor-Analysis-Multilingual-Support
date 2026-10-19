use crate::types::TokenizerName;
use crate::tokenizer::Tokenizer;
use std::collections::HashSet;

/// Simulates English-centric BPE tokenizer
/// English vocab present -> 1 token, non-English -> split into chars / high fertility
/// Demonstrates Proposition 3: ∃ l Fert(T,l) >> Fert(T,en)
pub struct SimpleBpeTokenizer {
    name: TokenizerName,
    vocab: HashSet<String>,
}

impl SimpleBpeTokenizer {
    pub fn new(name: &str, vocab_words: &[&str]) -> Self {
        let mut vocab = HashSet::with_capacity(vocab_words.len() * 2);
        for w in vocab_words {
            vocab.insert(w.to_string());
            vocab.insert(w.to_lowercase());
        }
        Self {
            name: TokenizerName::new(name).expect("valid name"),
            vocab,
        }
    }

    pub fn english_centric() -> Self {
        // Pre-allocate vocab to eliminate reallocation
        let en_vocab = [
            "the", "is", "a", "an", "and", "or", "in", "on", "of", "to", "for", "with", "hello", "world",
            "this", "that", "code", "function", "let", "var", "const", "if", "else", "return", "import",
            "def", "class", "int", "string", "print", "for", "while", "loop", "fn", "mut", "self",
            "I", "you", "we", "are", "be", "have", "has", "it", "not", "from", "as", "by", "at",
        ];
        Self::new("english_centric_bpe", &en_vocab)
    }

    fn word_in_vocab(&self, word: &str) -> bool {
        // Zero-copy check: check lowercased version without cloning if possible?
        // For demo we do lookup - Cow would be used if we conditionally lowercased
        self.vocab.contains(word) || self.vocab.contains(&word.to_lowercase())
    }
}

impl Tokenizer for SimpleBpeTokenizer {
    fn name(&self) -> &TokenizerName { &self.name }

    fn count_tokens(&self, text: &str) -> usize {
        // Iterator pipeline efficiency
        text.split_whitespace()
            .map(|w| self.count_tokens_for_word(w))
            .sum()
    }

    fn count_tokens_for_word(&self, word: &str) -> usize {
        // UTF-8 reality: char count, not byte len
        let char_len = word.chars().count();
        if char_len == 0 {
            return 0;
        }
        if self.word_in_vocab(word) {
            1
        } else {
            // Non-English: simulate BPE fragmentation
            // If contains non-ASCII -> higher fragmentation
            let has_non_ascii = word.chars().any(|c| !c.is_ascii());
            if has_non_ascii {
                // Simulate byte-level fallback: 2-3 tokens per char for non-latin
                // This reproduces high fertility for hi, ta, zh
                (char_len * 3).max(2)
            } else {
                // English OOV: split into ~ 2-3 pieces
                (char_len / 3).max(1).max(2)
            }
        }
    }

    fn encode_debug(&self, text: &str) -> Vec<String> {
        let mut tokens = Vec::with_capacity(text.split_whitespace().count() * 2);
        for word in text.split_whitespace() {
            if self.word_in_vocab(word) {
                tokens.push(word.to_string());
            } else {
                // fragmentation
                for c in word.chars() {
                    tokens.push(c.to_string());
                }
            }
        }
        tokens
    }
}
