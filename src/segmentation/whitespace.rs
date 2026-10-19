use crate::error::Result;
use crate::segmentation::WordSegmenter;

/// Whitespace + punctuation aware segmenter
/// Embraces UTF-8 reality: iterates over char, not bytes
pub struct WhitespaceSegmenter;

impl WordSegmenter for WhitespaceSegmenter {
    fn name(&self) -> &'static str { "whitespace" }

    fn segment<'a>(&self, text: &'a str) -> Result<Vec<&'a str>> {
        // Pre-allocate: heuristic ~ text.len()/5 words
        let mut words = Vec::with_capacity(text.len() / 5 + 1);

        // Zero-copy: split_on_whitespace returns &str slices
        // Further split on punctuation but keep slices via char_indices filtering
        for token in text.split_whitespace() {
            // Lean on iterators: filter punctuation via char iteration, not byte
            // We want to keep words like "don't" together but split commas
            let cleaned = token.trim_matches(|c: char| {
                // punctuation except those inside words
                matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '(' | ')' | '"' | '\'' | '`')
            });
            if !cleaned.is_empty() {
                // Handle code case: split on punctuation for sub-tokens? Keep simple for now
                // Further split if contains punctuation inside like "hello,world"
                // Use iterator lazy evaluation
                if cleaned.contains(|c: char| matches!(c, '/' | '=' | '{' | '}' | '(' | ')')) && cleaned.len() > 1 {
                    // For code, split on those boundaries but still zero-copy via manual slicing
                    // Simplistic: push as is, analyzer can handle
                    words.push(cleaned);
                } else {
                    words.push(cleaned);
                }
            }
        }
        Ok(words)
    }
}
