# Fertility Analyzer - Rust

Implements: Multilingual Fertility & Token Overhead Analysis

Objective: ∀ T ∃ S ⊆ L (Overhead(T,l) > τ) → flag high-overhead languages, find T* where ∀ l Overhead ≤ τ

Rules enforced (OR predicates):
- Zero-Copy First: &str slices over String cloning
- UTF-8 Reality: char / graphemes, not bytes
- Pre-allocate collections
- Domain constraints via Newtypes
- ? not unwrap
- Iterators + lazy eval
- No unsafe
- Cow for conditional normalization
- Intentional lifetimes
- Criterion benches

## Modules
- types: Newtypes LanguageCode, TokenizerName, Counters
- error: thiserror
- normalization: Cow<str> NFKC
- segmentation: WordSegmenter trait + whitespace + CJK
- tokenizer: Tokenizer trait + mock + simple_bpe
- corpus: Corpus loading
- metrics: accumulation + fertility/cpt/strr
- analyzer: double loop T × L
- report: ranking, overhead, CSV/JSON
- main: CLI demo with en, hi, zh, ta, code

Run:
```
cargo run -- --reference en --threshold 2.0
cargo bench
```
