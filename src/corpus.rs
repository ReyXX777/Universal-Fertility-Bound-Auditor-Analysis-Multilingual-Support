use crate::types::LanguageCode;
use std::collections::HashMap;

/// Corpus for a single language
#[derive(Debug, Clone)]
pub struct Corpus {
    pub language: LanguageCode,
    pub documents: Vec<String>,
}

impl Corpus {
    pub fn new(language: LanguageCode, documents: Vec<String>) -> Self {
        Self { language, documents }
    }

    pub fn doc_count(&self) -> usize { self.documents.len() }

    /// Iterator over &str slices - zero-copy
    pub fn iter_docs(&self) -> impl Iterator<Item=&str> {
        self.documents.iter().map(|s| s.as_str())
    }
}

/// Holds all corpora L = [lang1,...]
pub struct CorpusCollection {
    map: HashMap<String, Corpus>,
}

impl CorpusCollection {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn insert(&mut self, corpus: Corpus) {
        self.map.insert(corpus.language.as_str().to_string(), corpus);
    }

    pub fn get(&self, lang: &LanguageCode) -> Option<&Corpus> {
        self.map.get(lang.as_str())
    }

    pub fn languages(&self) -> Vec<LanguageCode> {
        self.map.values().map(|c| c.language.clone()).collect()
    }

    /// Sample corpora for assignment demo - parallel-ish sentences
    pub fn sample() -> Self {
        let mut col = Self::new();
        // Pre-allocate
        let mut docs_en = Vec::with_capacity(5);
        docs_en.push("Hello world this is a test of tokenization efficiency".to_string());
        docs_en.push("The quick brown fox jumps over the lazy dog and code function".to_string());
        docs_en.push("We need to analyze fertility for multilingual models".to_string());
        docs_en.push("fn main() { println!("hello"); let x = 5; }".to_string());
        docs_en.push("This is English and it should have low fertility".to_string());

        let mut docs_hi = Vec::with_capacity(5);
        docs_hi.push("नमस्ते दुनिया यह टोकनाइज़ेशन दक्षता का परीक्षण है".to_string());
        docs_hi.push("तेज़ भूरी लोमड़ी आलसी कुत्ते के ऊपर कूदती है".to_string());
        docs_hi.push("हमें बहुभाषी मॉडलों के लिए प्रजनन क्षमता का विश्लेषण करने की आवश्यकता है".to_string());
        docs_hi.push("यह हिंदी है और इसमें उच्च प्रजनन क्षमता होनी चाहिए".to_string());
        docs_hi.push("कोड फ़ंक्शन मुख्य प्रिंट".to_string());

        let mut docs_zh = Vec::with_capacity(5);
        docs_zh.push("你好世界 这是分词效率的测试".to_string());
        docs_zh.push("快速的棕色狐狸跳过懒狗".to_string());
        docs_zh.push("我们需要分析多语言模型的生育率".to_string());
        docs_zh.push("这是中文 应该有很高的生育率".to_string());
        docs_zh.push("代码 函数 主要 打印".to_string());

        let mut docs_ta = Vec::with_capacity(5);
        docs_ta.push("வணக்கம் உலகம் இது டோக்கனைசேஷன் செயல்திறனின் சோதனை".to_string());
        docs_ta.push("வேகமான பழுப்பு நரி சோம்பேறி நாய் மீது குதிக்கிறது".to_string());
        docs_ta.push("பன்மொழி மாதிரிகளுக்கான கருவுறுதலை நாம் பகுப்பாய்வு செய்ய வேண்டும்".to_string());
        docs_ta.push("இது தமிழ் அதிக கருவுறுதலைக் கொண்டிருக்க வேண்டும்".to_string());

        let mut docs_code = Vec::with_capacity(5);
        docs_code.push("def fibonacci(n): if n <= 1: return n else: return fibonacci(n-1) + fibonacci(n-2)".to_string());
        docs_code.push("fn main() { let mut x = vec![1,2,3]; x.iter().map(|v| v*2).collect::<Vec<_>>(); }".to_string());
        docs_code.push("import numpy as np; class Model: def __init__(self): self.weights = np.zeros(10)".to_string());
        docs_code.push("SELECT * FROM users WHERE language = 'en' AND fertility > 2.5".to_string());
        docs_code.push("for i in range(100): print(f"token {i}")".to_string());

        col.insert(Corpus::new(LanguageCode::new("en").unwrap(), docs_en));
        col.insert(Corpus::new(LanguageCode::new("hi").unwrap(), docs_hi));
        col.insert(Corpus::new(LanguageCode::new("zh").unwrap(), docs_zh));
        col.insert(Corpus::new(LanguageCode::new("ta").unwrap(), docs_ta));
        col.insert(Corpus::new(LanguageCode::new("code").unwrap(), docs_code));
        col
    }
}
