use std::borrow::Cow;
use unicode_normalization::UnicodeNormalization;

/// Conditional normalization using Cow
/// Returns Borrowed if already NFKC, Owned if transformation needed
/// Zero-copy first principle
pub fn normalize_cow(text: &str) -> Cow<'_, str> {
    // Fast path: check if already NFKC? We compute normalized and compare.
    // Cow avoids allocation when no change.
    let normalized: String = text.nfkc().collect();
    if normalized == text {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(normalized)
    }
}

/// Count characters (UTF-8 aware) - not bytes
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Count graphemes (user-perceived characters)
pub fn grapheme_count(s: &str) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    s.graphemes(true).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cow_borrowed_when_no_change() {
        let s = "hello";
        let c = normalize_cow(s);
        assert!(matches!(c, Cow::Borrowed(_)));
    }
}
