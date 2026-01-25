//! Chinese word segmentation using jieba-rs
//!
//! This module wraps jieba-rs to provide word segmentation with POS tagging
//! for Chinese text.

use jieba_rs::Jieba;
use once_cell::sync::Lazy;

/// Global jieba instance for segmentation
static JIEBA: Lazy<Jieba> = Lazy::new(Jieba::new);

/// Word segmenter with POS tagging support
pub struct Segmenter {
    // We use the global JIEBA instance
}

impl Segmenter {
    /// Create a new segmenter
    pub fn new() -> Self {
        // Force initialization of JIEBA
        let _ = &*JIEBA;
        Self {}
    }

    /// Segment text into words without POS tags
    pub fn segment(&self, text: &str) -> Vec<String> {
        JIEBA
            .cut(text, false)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Segment text into words with POS tags
    /// Returns a vector of (word, POS tag) tuples
    pub fn segment_with_pos(&self, text: &str) -> Vec<(String, String)> {
        JIEBA
            .tag(text, false)
            .into_iter()
            .map(|t| (t.word.to_string(), t.tag.to_string()))
            .collect()
    }

    /// Check if a character is a Chinese character (CJK Unified Ideograph)
    #[inline]
    pub fn is_chinese_char(c: char) -> bool {
        matches!(c,
            '\u{4E00}'..='\u{9FFF}' |   // CJK Unified Ideographs
            '\u{3400}'..='\u{4DBF}' |   // CJK Unified Ideographs Extension A
            '\u{20000}'..='\u{2A6DF}' | // CJK Unified Ideographs Extension B
            '\u{2A700}'..='\u{2B73F}' | // CJK Unified Ideographs Extension C
            '\u{2B740}'..='\u{2B81F}' | // CJK Unified Ideographs Extension D
            '\u{F900}'..='\u{FAFF}' |   // CJK Compatibility Ideographs
            '\u{2F800}'..='\u{2FA1F}'   // CJK Compatibility Ideographs Supplement
        )
    }

    /// Check if a string contains any Chinese characters
    pub fn contains_chinese(text: &str) -> bool {
        text.chars().any(Self::is_chinese_char)
    }
}

impl Default for Segmenter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_basic() {
        let seg = Segmenter::new();
        let words = seg.segment("我爱北京天安门");
        assert!(words.len() > 1);
        assert!(words.contains(&"我".to_string()) || words.join("").contains("我"));
    }

    #[test]
    fn test_segment_with_pos() {
        let seg = Segmenter::new();
        let tagged = seg.segment_with_pos("我爱中国");
        assert!(!tagged.is_empty());
        // Check that we have POS tags
        for (word, pos) in &tagged {
            assert!(!word.is_empty());
            assert!(!pos.is_empty());
        }
    }

    #[test]
    fn test_is_chinese_char() {
        assert!(Segmenter::is_chinese_char('中'));
        assert!(Segmenter::is_chinese_char('国'));
        assert!(!Segmenter::is_chinese_char('a'));
        assert!(!Segmenter::is_chinese_char('1'));
    }

    #[test]
    fn test_contains_chinese() {
        assert!(Segmenter::contains_chinese("Hello 世界"));
        assert!(!Segmenter::contains_chinese("Hello World"));
    }
}
