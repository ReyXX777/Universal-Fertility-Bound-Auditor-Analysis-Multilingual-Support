use crate::corpus::CorpusCollection;
use crate::metrics::{FertilityMetrics, RawAccumulator};
use crate::normalization::{char_count, grapheme_count, normalize_cow};
use crate::segmentation::segmenter_for_language;
use crate::tokenizer::Tokenizer;
use crate::types::{LanguageCode, OverheadThreshold};

/// Core analyzer - implements PROCEDURE from algorithm
pub struct FertilityAnalyzer<'a> {
    corpora: &'a CorpusCollection,
    tokenizers: Vec<Box<dyn Tokenizer>>,
}

impl<'a> FertilityAnalyzer<'a> {
    pub fn new(corpora: &'a CorpusCollection, tokenizers: Vec<Box<dyn Tokenizer>>) -> Self {
        Self { corpora, tokenizers }
    }

    /// Main double loop: FOR each tokenizer, FOR each language
    pub fn analyze(&self) -> Vec<FertilityMetrics> {
        let mut results = Vec::with_capacity(self.tokenizers.len() * 5);

        for tok in &self.tokenizers {
            // For each language in collection
            for lang in self.corpora.languages() {
                if let Some(corpus) = self.corpora.get(&lang) {
                    let metrics = self.analyze_pair(tok.as_ref(), &lang, corpus);
                    results.push(metrics);
                }
            }
        }
        results
    }

    fn analyze_pair(&self, tok: &dyn Tokenizer, lang: &LanguageCode, corpus: &crate::corpus::Corpus) -> FertilityMetrics {
        let mut acc = RawAccumulator {
            tokenizer: crate::types::TokenizerName::new(tok.name().as_str()).unwrap(),
            language: lang.clone(),
            total_tokens: 0,
            total_words: 0,
            total_chars: 0,
            total_graphemes: 0,
            single_token_words: 0,
        };

        let segmenter = segmenter_for_language(lang.as_str());

        // FOR each document
        for doc in corpus.iter_docs() {
            // 1. Normalize with Cow - zero-copy if already normalized
            let normalized = normalize_cow(doc);

            // 2. Obtain word list - zero-copy slices borrowing from normalized
            let words = match segmenter.segment(&normalized) {
                Ok(w) => w,
                Err(_) => Vec::new(),
            };

            // 3. Tokenize original text (preferred: full sequence)
            let num_tokens = tok.count_tokens(&normalized);

            // UTF-8 reality: char and grapheme counts, not bytes
            let chars = char_count(&normalized) as u64;
            let graphemes = grapheme_count(&normalized) as u64;

            // STRR calculation - iterator pipeline, lazy evaluation
            let single_count = words.iter()
                .filter(|w| tok.is_single_token(w))
                .count() as u64;

            acc.add_doc(words.len() as u64, num_tokens as u64, chars, graphemes, single_count);
        }

        acc.finalize()
    }

    /// Compute overhead vs reference language and flag high overhead
    pub fn compute_overhead(&self, mut metrics: Vec<FertilityMetrics>, reference: &LanguageCode, _threshold: OverheadThreshold) -> Vec<FertilityMetrics> {
        // Build map reference fertility per tokenizer
        use std::collections::HashMap;
        let mut ref_map: HashMap<String, f64> = HashMap::with_capacity(self.tokenizers.len());

        for m in &metrics {
            if m.language == reference.as_str() {
                ref_map.insert(m.tokenizer.clone(), m.fertility);
            }
        }

        for m in &mut metrics {
            if let Some(ref_fert) = ref_map.get(&m.tokenizer) {
                if *ref_fert > 0.0 {
                    m.overhead_vs_reference = Some(m.fertility / ref_fert);
                }
            }
        }
        metrics
    }
}
