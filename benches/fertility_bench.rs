use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fertility_analyzer::{
    corpus::CorpusCollection,
    tokenizer::{self, Tokenizer},
    segmentation::{WhitespaceSegmenter, CjkSegmenter, WordSegmenter},
    normalization::normalize_cow,
};

fn bench_segmentation(c: &mut Criterion) {
    let ws = WhitespaceSegmenter;
    let cjk = CjkSegmenter;
    let text_en = "Hello world this is a test of tokenization efficiency for multilingual models";
    let text_hi = "नमस्ते दुनिया यह टोकनाइज़ेशन दक्षता का परीक्षण है बहुभाषी मॉडल के लिए";
    let text_zh = "你好世界 这是分词效率的测试 多语言模型的生育率分析";

    c.bench_function("segment_en_whitespace", |b| {
        b.iter(|| black_box(ws.segment(black_box(text_en)).unwrap()))
    });
    c.bench_function("segment_hi_whitespace", |b| {
        b.iter(|| black_box(ws.segment(black_box(text_hi)).unwrap()))
    });
    c.bench_function("segment_zh_cjk", |b| {
        b.iter(|| black_box(cjk.segment(black_box(text_zh)).unwrap()))
    });
}

fn bench_tokenization(c: &mut Criterion) {
    let tokenizers = tokenizer::create_tokenizers();
    let text = "Hello world नमस्ते दुनिया 你好世界 code fn main()";

    for tok in tokenizers {
        let name = tok.name().as_str().to_string();
        c.bench_function(&format!("tokenize_{}", name), |b| {
            b.iter(|| black_box(tok.count_tokens(black_box(text))))
        });
    }
}

fn bench_normalization(c: &mut Criterion) {
    let text = "Café naïve résumé";
    c.bench_function("normalize_cow", |b| {
        b.iter(|| black_box(normalize_cow(black_box(text))))
    });
}

fn bench_full_analyze(c: &mut Criterion) {
    let corpora = CorpusCollection::sample();
    let tokenizers = tokenizer::create_tokenizers();
    c.bench_function("full_analyze", |b| {
        b.iter(|| {
            let analyzer = fertility_analyzer::analyzer::FertilityAnalyzer::new(&corpora, tokenizer::create_tokenizers());
            black_box(analyzer.analyze())
        })
    });
}

criterion_group!(benches, bench_segmentation, bench_tokenization, bench_normalization, bench_full_analyze);
criterion_main!(benches);
