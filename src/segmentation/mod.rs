pub mod whitespace;
pub mod cjk;

use crate::error::Result;

pub trait WordSegmenter: Send + Sync {
    /// Zero-copy: returns slices borrowing from input text
    /// Lifetime intentionally tied to input
    fn segment<'a>(&self, text: &'a str) -> Result<Vec<&'a str>>;

    fn name(&self) -> &'static str;
}

pub use whitespace::WhitespaceSegmenter;
pub use cjk::CjkSegmenter;

/// Factory that selects segmenter by language code
pub fn segmenter_for_language(lang: &str) -> Box<dyn WordSegmenter> {
    match lang {
        "zh" | "ja" | "th" | "lo" | "my" | "km" => Box::new(CjkSegmenter),
        _ => Box::new(WhitespaceSegmenter),
    }
}
