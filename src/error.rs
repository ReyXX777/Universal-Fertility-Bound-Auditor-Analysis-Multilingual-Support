use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error("invalid language code: {0}")]
    InvalidLanguageCode(String),
    #[error("invalid tokenizer name: {0}")]
    InvalidTokenizerName(String),
    #[error("empty corpus for language {0}")]
    EmptyCorpus(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("segmentation error: {0}")]
    Segmentation(String),
    #[error("tokenization error: {0}")]
    Tokenization(String),
    #[error("config error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, AnalyzerError>;
