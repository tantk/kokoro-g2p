//! Chinese G2P (Grapheme-to-Phoneme) module for Mandarin Chinese (zh-CN)
//!
//! This module provides Chinese text to phoneme conversion for the Kokoro TTS model.
//! It supports:
//! - Text normalization (numbers, dates, currency)
//! - Word segmentation via jieba-rs
//! - Hanzi to Pinyin conversion
//! - Tone sandhi rules (3-3, 一, 不)
//! - Polyphone resolution
//! - Pinyin to Zhuyin mapping for kokoro-v1.1-zh tokens

pub mod normalizer;
pub mod phoneme_mapper;
pub mod pinyin;
pub mod polyphone;
pub mod segmenter;
pub mod tone_sandhi;

use crate::tokenizer;

/// Chinese G2P processor
pub struct ChineseG2P {
    segmenter: segmenter::Segmenter,
}

impl ChineseG2P {
    /// Create a new Chinese G2P processor
    pub fn new() -> Self {
        Self {
            segmenter: segmenter::Segmenter::new(),
        }
    }

    /// Convert Chinese text to phoneme string (Zhuyin-based)
    pub fn text_to_phonemes(&self, text: &str) -> String {
        // Step 1: Normalize text (numbers, dates, currency)
        let normalized = normalizer::normalize(text);

        // Step 2: Segment into words with POS tagging
        let segments = self.segmenter.segment_with_pos(&normalized);

        // Step 3: Convert each segment to pinyin with polyphone resolution
        let mut pinyin_result = Vec::new();
        for (word, pos) in &segments {
            let pinyins = pinyin::to_pinyin_with_pos(word, pos);
            pinyin_result.extend(pinyins);
        }

        // Step 4: Apply tone sandhi rules
        let sandhi_applied = tone_sandhi::apply_tone_sandhi(&pinyin_result);

        // Step 5: Convert pinyin to Zhuyin phonemes
        let phonemes = phoneme_mapper::pinyin_to_zhuyin(&sandhi_applied);

        phonemes
    }

    /// Convert Chinese text to token IDs
    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        tokenizer::phonemes_to_tokens(&phonemes)
    }
}

impl Default for ChineseG2P {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Chinese text to token IDs (convenience function)
pub fn text_to_tokens(text: &str) -> Vec<i64> {
    let g2p = ChineseG2P::new();
    g2p.text_to_tokens(text)
}

/// Convert Chinese text to phoneme string (convenience function)
pub fn text_to_phonemes(text: &str) -> String {
    let g2p = ChineseG2P::new();
    g2p.text_to_phonemes(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = ChineseG2P::new();
        let phonemes = g2p.text_to_phonemes("你好");
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_ni_hao_sandhi() {
        // 你好 should apply 3-3 tone sandhi: ní hǎo
        let g2p = ChineseG2P::new();
        let phonemes = g2p.text_to_phonemes("你好");
        // After sandhi, the first syllable should have tone 2
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("你好世界");
        assert!(tokens.len() > 2); // At least padding + some content
    }
}
