//! Unified KPipeline for multi-language G2P processing
//!
//! The KPipeline provides a unified interface for converting text to phonemes
//! and tokens across different languages.

use crate::tokenizer;

#[cfg(feature = "english")]
use crate::g2p::G2P;

#[cfg(feature = "chinese")]
use crate::zh::ChineseG2P;

#[cfg(feature = "japanese")]
use crate::ja::JapaneseG2P;

#[cfg(feature = "spanish")]
use crate::es::SpanishG2P;

#[cfg(feature = "indonesian")]
use crate::id::IndonesianG2P;

#[cfg(feature = "turkish")]
use crate::tr::TurkishG2P;

#[cfg(feature = "italian")]
use crate::it::ItalianG2P;

#[cfg(feature = "german")]
use crate::de::GermanG2P;

#[cfg(feature = "portuguese")]
use crate::pt::PortugueseG2P;

#[cfg(feature = "korean")]
use crate::ko::KoreanG2P;

#[cfg(feature = "vietnamese")]
use crate::vi::VietnameseG2P;

/// Result of G2P processing
#[derive(Debug, Clone)]
pub struct G2PResult {
    /// Phoneme string representation
    pub phonemes: String,
    /// Token IDs for the TTS model
    pub tokens: Vec<i64>,
}

/// Language configuration for the pipeline
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    /// American English
    EnglishUS,
    /// British English
    EnglishGB,
    /// Mandarin Chinese
    Chinese,
    /// Japanese
    Japanese,
    /// Spanish
    Spanish,
    /// Indonesian
    Indonesian,
    /// Turkish
    Turkish,
    /// Italian
    Italian,
    /// German
    German,
    /// Portuguese
    Portuguese,
    /// Korean
    Korean,
    /// Vietnamese
    Vietnamese,
}

impl Language {
    /// Parse a language string into a Language enum
    pub fn from_str(s: &str) -> Self {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "en-gb" | "british" | "gb" => Language::EnglishGB,
            "zh" | "zh-cn" | "chinese" | "mandarin" | "cmn" => Language::Chinese,
            "ja" | "jp" | "japanese" | "日本語" => Language::Japanese,
            "es" | "es-es" | "es-mx" | "spanish" | "español" => Language::Spanish,
            "id" | "indonesian" | "bahasa" => Language::Indonesian,
            "tr" | "turkish" | "türkçe" => Language::Turkish,
            "it" | "italian" | "italiano" => Language::Italian,
            "de" | "german" | "deutsch" => Language::German,
            "pt" | "pt-br" | "pt-pt" | "portuguese" | "português" => Language::Portuguese,
            "ko" | "korean" | "한국어" => Language::Korean,
            "vi" | "vietnamese" | "tiếng việt" => Language::Vietnamese,
            _ => Language::EnglishUS, // Default to US English
        }
    }

    /// Get the language code string
    pub fn code(&self) -> &'static str {
        match self {
            Language::EnglishUS => "en-us",
            Language::EnglishGB => "en-gb",
            Language::Chinese => "zh",
            Language::Japanese => "ja",
            Language::Spanish => "es",
            Language::Indonesian => "id",
            Language::Turkish => "tr",
            Language::Italian => "it",
            Language::German => "de",
            Language::Portuguese => "pt",
            Language::Korean => "ko",
            Language::Vietnamese => "vi",
        }
    }
}

/// Unified G2P pipeline supporting multiple languages
///
/// The KPipeline lazily initializes language-specific G2P engines as needed.
pub struct KPipeline {
    language: Language,
    #[cfg(feature = "english")]
    english_g2p: Option<G2P>,
    #[cfg(feature = "chinese")]
    chinese_g2p: Option<ChineseG2P>,
    #[cfg(feature = "japanese")]
    japanese_g2p: Option<JapaneseG2P>,
    #[cfg(feature = "spanish")]
    spanish_g2p: Option<SpanishG2P>,
    #[cfg(feature = "indonesian")]
    indonesian_g2p: Option<IndonesianG2P>,
    #[cfg(feature = "turkish")]
    turkish_g2p: Option<TurkishG2P>,
    #[cfg(feature = "italian")]
    italian_g2p: Option<ItalianG2P>,
    #[cfg(feature = "german")]
    german_g2p: Option<GermanG2P>,
    #[cfg(feature = "portuguese")]
    portuguese_g2p: Option<PortugueseG2P>,
    #[cfg(feature = "korean")]
    korean_g2p: Option<KoreanG2P>,
    #[cfg(feature = "vietnamese")]
    vietnamese_g2p: Option<VietnameseG2P>,
}

impl KPipeline {
    /// Create a new pipeline for the specified language
    pub fn new(language: &str) -> Self {
        let lang = Language::from_str(language);
        Self {
            language: lang,
            #[cfg(feature = "english")]
            english_g2p: None,
            #[cfg(feature = "chinese")]
            chinese_g2p: None,
            #[cfg(feature = "japanese")]
            japanese_g2p: None,
            #[cfg(feature = "spanish")]
            spanish_g2p: None,
            #[cfg(feature = "indonesian")]
            indonesian_g2p: None,
            #[cfg(feature = "turkish")]
            turkish_g2p: None,
            #[cfg(feature = "italian")]
            italian_g2p: None,
            #[cfg(feature = "german")]
            german_g2p: None,
            #[cfg(feature = "portuguese")]
            portuguese_g2p: None,
            #[cfg(feature = "korean")]
            korean_g2p: None,
            #[cfg(feature = "vietnamese")]
            vietnamese_g2p: None,
        }
    }

    /// Create a new pipeline with default language (US English)
    pub fn default() -> Self {
        Self::new("en-us")
    }

    /// Get the current language
    pub fn language(&self) -> &Language {
        &self.language
    }

    /// Set the language for subsequent processing
    pub fn set_language(&mut self, language: &str) {
        self.language = Language::from_str(language);
    }

    /// Process text and return both phonemes and tokens
    pub fn process(&mut self, text: &str) -> G2PResult {
        match &self.language {
            #[cfg(feature = "chinese")]
            Language::Chinese => {
                if self.chinese_g2p.is_none() {
                    self.chinese_g2p = Some(ChineseG2P::new());
                }
                let g2p = self.chinese_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "chinese"))]
            Language::Chinese => {
                log::warn!("Chinese language requested but 'chinese' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "spanish")]
            Language::Spanish => {
                if self.spanish_g2p.is_none() {
                    self.spanish_g2p = Some(SpanishG2P::new());
                }
                let g2p = self.spanish_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "spanish"))]
            Language::Spanish => {
                log::warn!("Spanish language requested but 'spanish' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "indonesian")]
            Language::Indonesian => {
                if self.indonesian_g2p.is_none() {
                    self.indonesian_g2p = Some(IndonesianG2P::new());
                }
                let g2p = self.indonesian_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "indonesian"))]
            Language::Indonesian => {
                log::warn!("Indonesian language requested but 'indonesian' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "turkish")]
            Language::Turkish => {
                if self.turkish_g2p.is_none() {
                    self.turkish_g2p = Some(TurkishG2P::new());
                }
                let g2p = self.turkish_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "turkish"))]
            Language::Turkish => {
                log::warn!("Turkish language requested but 'turkish' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "italian")]
            Language::Italian => {
                if self.italian_g2p.is_none() {
                    self.italian_g2p = Some(ItalianG2P::new());
                }
                let g2p = self.italian_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "italian"))]
            Language::Italian => {
                log::warn!("Italian language requested but 'italian' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "german")]
            Language::German => {
                if self.german_g2p.is_none() {
                    self.german_g2p = Some(GermanG2P::new());
                }
                let g2p = self.german_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "german"))]
            Language::German => {
                log::warn!("German language requested but 'german' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "portuguese")]
            Language::Portuguese => {
                if self.portuguese_g2p.is_none() {
                    self.portuguese_g2p = Some(PortugueseG2P::new());
                }
                let g2p = self.portuguese_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "portuguese"))]
            Language::Portuguese => {
                log::warn!("Portuguese language requested but 'portuguese' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "korean")]
            Language::Korean => {
                if self.korean_g2p.is_none() {
                    self.korean_g2p = Some(KoreanG2P::new());
                }
                let g2p = self.korean_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "korean"))]
            Language::Korean => {
                log::warn!("Korean language requested but 'korean' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "japanese")]
            Language::Japanese => {
                if self.japanese_g2p.is_none() {
                    self.japanese_g2p = Some(JapaneseG2P::new());
                }
                let g2p = self.japanese_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "japanese"))]
            Language::Japanese => {
                log::warn!("Japanese language requested but 'japanese' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => {
                if self.vietnamese_g2p.is_none() {
                    self.vietnamese_g2p = Some(VietnameseG2P::new());
                }
                let g2p = self.vietnamese_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "vietnamese"))]
            Language::Vietnamese => {
                log::warn!("Vietnamese language requested but 'vietnamese' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
            #[cfg(feature = "english")]
            Language::EnglishUS | Language::EnglishGB => {
                let british = matches!(self.language, Language::EnglishGB);
                if self.english_g2p.is_none() || self.english_g2p.as_ref().map(|g| g.is_british()) != Some(british) {
                    self.english_g2p = Some(G2P::new(british));
                }
                let g2p = self.english_g2p.as_ref().unwrap();
                let phonemes = g2p.text_to_phonemes(text);
                let tokens = tokenizer::phonemes_to_tokens(&phonemes);
                G2PResult { phonemes, tokens }
            }
            #[cfg(not(feature = "english"))]
            Language::EnglishUS | Language::EnglishGB => {
                log::warn!("English language requested but 'english' feature not enabled");
                G2PResult {
                    phonemes: String::new(),
                    tokens: vec![tokenizer::PAD_TOKEN, tokenizer::PAD_TOKEN],
                }
            }
        }
    }

    /// Convert text to phonemes only
    pub fn to_phonemes(&mut self, text: &str) -> String {
        self.process(text).phonemes
    }

    /// Convert text to tokens only
    pub fn to_tokens(&mut self, text: &str) -> Vec<i64> {
        self.process(text).tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "english")]
    fn test_pipeline_english() {
        let mut pipeline = KPipeline::new("en-us");
        let result = pipeline.process("Hello, world!");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_pipeline_chinese() {
        let mut pipeline = KPipeline::new("zh");
        let result = pipeline.process("你好");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    fn test_language_parsing() {
        assert_eq!(Language::from_str("en-us"), Language::EnglishUS);
        assert_eq!(Language::from_str("en-gb"), Language::EnglishGB);
        assert_eq!(Language::from_str("zh"), Language::Chinese);
        assert_eq!(Language::from_str("chinese"), Language::Chinese);
        assert_eq!(Language::from_str("es"), Language::Spanish);
        assert_eq!(Language::from_str("spanish"), Language::Spanish);
        assert_eq!(Language::from_str("id"), Language::Indonesian);
        assert_eq!(Language::from_str("indonesian"), Language::Indonesian);
        assert_eq!(Language::from_str("tr"), Language::Turkish);
        assert_eq!(Language::from_str("turkish"), Language::Turkish);
        assert_eq!(Language::from_str("it"), Language::Italian);
        assert_eq!(Language::from_str("italian"), Language::Italian);
        assert_eq!(Language::from_str("de"), Language::German);
        assert_eq!(Language::from_str("german"), Language::German);
        assert_eq!(Language::from_str("pt"), Language::Portuguese);
        assert_eq!(Language::from_str("portuguese"), Language::Portuguese);
        assert_eq!(Language::from_str("ko"), Language::Korean);
        assert_eq!(Language::from_str("korean"), Language::Korean);
        assert_eq!(Language::from_str("vi"), Language::Vietnamese);
        assert_eq!(Language::from_str("vietnamese"), Language::Vietnamese);
        assert_eq!(Language::from_str("unknown"), Language::EnglishUS);
    }

    #[test]
    #[cfg(feature = "spanish")]
    fn test_pipeline_spanish() {
        let mut pipeline = KPipeline::new("es");
        let result = pipeline.process("hola mundo");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    #[cfg(feature = "indonesian")]
    fn test_pipeline_indonesian() {
        let mut pipeline = KPipeline::new("id");
        let result = pipeline.process("selamat pagi");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    #[cfg(feature = "turkish")]
    fn test_pipeline_turkish() {
        let mut pipeline = KPipeline::new("tr");
        let result = pipeline.process("merhaba dünya");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    #[cfg(feature = "italian")]
    fn test_pipeline_italian() {
        let mut pipeline = KPipeline::new("it");
        let result = pipeline.process("ciao mondo");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    fn test_language_switch() {
        let mut pipeline = KPipeline::new("en-us");
        assert_eq!(pipeline.language(), &Language::EnglishUS);

        pipeline.set_language("zh");
        assert_eq!(pipeline.language(), &Language::Chinese);
    }

    #[test]
    #[cfg(feature = "german")]
    fn test_pipeline_german() {
        let mut pipeline = KPipeline::new("de");
        let result = pipeline.process("Guten Tag");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    #[cfg(feature = "portuguese")]
    fn test_pipeline_portuguese() {
        let mut pipeline = KPipeline::new("pt");
        let result = pipeline.process("olá mundo");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    #[cfg(feature = "korean")]
    fn test_pipeline_korean() {
        let mut pipeline = KPipeline::new("ko");
        let result = pipeline.process("안녕하세요");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    #[test]
    #[cfg(feature = "vietnamese")]
    fn test_pipeline_vietnamese() {
        let mut pipeline = KPipeline::new("vi");
        let result = pipeline.process("xin chào");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }
}
