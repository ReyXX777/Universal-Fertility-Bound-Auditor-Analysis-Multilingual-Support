use crate::error::Result;
use crate::segmentation::WordSegmenter;
use unicode_segmentation::UnicodeSegmentation;

/// CJK / Thai segmenter - mock implementation
/// Real implementation would use jieba, MeCab, etc.
/// Here we use grapheme / char slices to demonstrate UTF-8 reality and zero-copy
pub struct CjkSegmenter;

impl WordSegmenter for CjkSegmenter {
    fn name(&self) -> &'static str { "cjk_char" }

    fn segment<'a>(&self, text: &'a str) -> Result<Vec<&'a str>> {
        // Pre-allocate: one per char heuristic
        let mut words = Vec::with_capacity(text.chars().count());

        // For CJK, no spaces. We segment by grapheme clusters, ignoring whitespace
        // Returns &str slices that borrow from original text
        for g in text.graphemes(true) {
            if g.trim().is_empty() {
                continue;
            }
            // Skip punctuation? keep as word for simplicity
            words.push(g);
        }
        Ok(words)
    }
}
