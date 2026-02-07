//! Japanese G2P (Grapheme-to-Phoneme) module for Kokoro TTS.
//!
//! This module converts Japanese text to IPA phonemes suitable for the Kokoro TTS model.
//!
//! # Pipeline
//!
//! 1. Text normalization (punctuation, numbers, etc.)
//! 2. Word segmentation and reading lookup
//! 3. Katakana to IPA phoneme conversion
//! 4. Token ID generation
//!
//! # Example
//!
//! ```rust
//! use kokoro_g2p::ja;
//!
//! let phonemes = ja::text_to_phonemes("こんにちは");
//! let tokens = ja::text_to_tokens("今日は元気ですか");
//! ```

mod phoneme_map;
mod reading;

pub use phoneme_map::{
    hiragana_to_katakana, is_hiragana, is_katakana, str_hiragana_to_katakana,
    COMBINED_KANA, PUNCT_MAP, SINGLE_KANA,
};
pub use reading::{get_reading, get_single_kanji_reading, is_kanji, READINGS};

use crate::tokenizer::{phonemes_to_tokens, PAD_TOKEN};

/// Japanese G2P processor
pub struct JapaneseG2P;

impl JapaneseG2P {
    pub fn new() -> Self {
        Self
    }

    pub fn text_to_phonemes(&self, text: &str) -> String {
        text_to_phonemes(text)
    }

    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        text_to_tokens(text)
    }
}

impl Default for JapaneseG2P {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Japanese text to IPA phoneme string.
///
/// # Example
///
/// ```rust
/// use kokoro_g2p::ja::text_to_phonemes;
///
/// let phonemes = text_to_phonemes("こんにちは");
/// assert!(!phonemes.is_empty());
/// ```
pub fn text_to_phonemes(text: &str) -> String {
    let normalized = normalize_text(text);
    let katakana = to_katakana_reading(&normalized);
    katakana_to_phonemes(&katakana)
}

/// Convert Japanese text to token IDs for Kokoro TTS.
///
/// Returns a vector of token IDs with padding tokens at start and end.
pub fn text_to_tokens(text: &str) -> Vec<i64> {
    let phonemes = text_to_phonemes(text);
    if phonemes.is_empty() {
        return vec![PAD_TOKEN, PAD_TOKEN];
    }
    phonemes_to_tokens(&phonemes)
}

/// Normalize Japanese text.
///
/// - Convert full-width ASCII to half-width
/// - Convert Japanese punctuation to ASCII
/// - Handle numbers
fn normalize_text(text: &str) -> String {
    let mut result = String::with_capacity(text.len());

    for c in text.chars() {
        // Convert Japanese punctuation
        if let Some(&ascii) = PUNCT_MAP.get(&c) {
            result.push(ascii);
            continue;
        }

        // Convert full-width ASCII to half-width (A-Z, a-z, 0-9)
        let normalized = match c {
            '\u{FF01}'..='\u{FF5E}' => {
                // Full-width ASCII range
                char::from_u32(c as u32 - 0xFEE0).unwrap_or(c)
            }
            _ => c,
        };

        result.push(normalized);
    }

    result
}

/// Convert text to katakana reading.
///
/// Handles:
/// - Hiragana -> Katakana
/// - Kanji -> Katakana (via dictionary lookup)
/// - Preserves punctuation
fn to_katakana_reading(text: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Check for multi-character dictionary matches (longest match first)
        let mut found = false;
        for len in (2..=4).rev() {
            if i + len <= chars.len() {
                let substr: String = chars[i..i + len].iter().collect();
                if let Some(reading) = get_reading(&substr) {
                    result.push_str(reading);
                    i += len;
                    found = true;
                    break;
                }
            }
        }
        if found {
            continue;
        }

        // Single character handling
        if is_hiragana(c) {
            result.push(hiragana_to_katakana(c));
        } else if is_katakana(c) {
            result.push(c);
        } else if is_kanji(c) {
            // Try single kanji reading
            if let Some(reading) = get_single_kanji_reading(c) {
                result.push_str(reading);
            } else {
                log::debug!("Unknown kanji: {}", c);
                result.push(c); // Preserve unknown kanji as-is
            }
        } else {
            // Punctuation, numbers, etc.
            result.push(c);
        }

        i += 1;
    }

    result
}

/// Convert katakana string to IPA phonemes.
fn katakana_to_phonemes(katakana: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = katakana.chars().collect();
    let mut i = 0;
    let mut last_was_vowel = false;

    while i < chars.len() {
        let c = chars[i];

        // Try two-character combination first
        if i + 1 < chars.len() {
            let pair: String = [c, chars[i + 1]].iter().collect();
            if let Some(&phoneme) = COMBINED_KANA.get(pair.as_str()) {
                // Add space before word if previous was vowel and this starts with consonant
                if last_was_vowel && !phoneme.starts_with(|c: char| "aeiou".contains(c)) {
                    // Could add prosody handling here
                }
                result.push_str(phoneme);
                last_was_vowel = phoneme.ends_with(|c: char| "aeiou".contains(c));
                i += 2;
                continue;
            }
        }

        // Single character
        if let Some(&phoneme) = SINGLE_KANA.get(&c) {
            result.push_str(phoneme);
            last_was_vowel = phoneme.ends_with(|c: char| "aeiouː".contains(c));
        } else if c.is_ascii_punctuation() || PUNCT_MAP.values().any(|&v| v == c) {
            // Keep punctuation
            result.push(c);
            last_was_vowel = false;
        } else if c == ' ' {
            result.push(' ');
            last_was_vowel = false;
        } else if c.is_ascii_alphanumeric() {
            // Pass through ASCII (numbers, romanji)
            result.push(c);
            last_was_vowel = "aeiouAEIOU".contains(c);
        }
        // Skip unknown characters

        i += 1;
    }

    result
}

/// Process text for debugging - returns intermediate representations
pub fn debug_process(text: &str) -> (String, String, String) {
    let normalized = normalize_text(text);
    let katakana = to_katakana_reading(&normalized);
    let phonemes = katakana_to_phonemes(&katakana);
    (normalized, katakana, phonemes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        assert_eq!(normalize_text("こんにちは。"), "こんにちは.");
        assert_eq!(normalize_text("ＡＢＣＤ"), "ABCD");
    }

    #[test]
    fn test_hiragana_conversion() {
        let (_, katakana, _) = debug_process("こんにちは");
        assert_eq!(katakana, "コンニチハ");
    }

    #[test]
    fn test_basic_phonemes() {
        let phonemes = text_to_phonemes("こんにちは");
        println!("こんにちは -> {}", phonemes);
        assert!(phonemes.contains("ko"));
        assert!(phonemes.contains("ɴ")); // ん -> ɴ
    }

    #[test]
    fn test_kanji_reading() {
        let phonemes = text_to_phonemes("今日");
        println!("今日 -> {}", phonemes);
        // 今日 -> キョウ -> ᶄoːu or similar
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_mixed_text() {
        let phonemes = text_to_phonemes("私は学生です。");
        println!("私は学生です。 -> {}", phonemes);
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_tokens() {
        let tokens = text_to_tokens("こんにちは");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
        println!("Tokens: {:?}", tokens);
    }

    #[test]
    fn test_combined_kana() {
        // シャ -> ɕa
        let phonemes = katakana_to_phonemes("シャ");
        assert_eq!(phonemes, "ɕa");

        // チュ -> ʨu
        let phonemes = katakana_to_phonemes("チュ");
        assert_eq!(phonemes, "ʨu");

        // ニャ -> ɲa
        let phonemes = katakana_to_phonemes("ニャ");
        assert_eq!(phonemes, "ɲa");
    }

    #[test]
    fn test_special_characters() {
        // ッ (geminate) -> ʔ
        let phonemes = katakana_to_phonemes("ガッコウ");
        assert!(phonemes.contains("ʔ"));

        // ン (nasal) -> ɴ
        let phonemes = katakana_to_phonemes("コン");
        assert!(phonemes.contains("ɴ"));

        // ー (long vowel) -> ː
        let phonemes = katakana_to_phonemes("コーヒー");
        assert!(phonemes.contains("ː"));
    }

    #[test]
    fn test_world() {
        let (_, _, phonemes) = debug_process("世界");
        println!("世界 -> {}", phonemes);
        // 世界 -> セカイ -> sekai
        assert!(phonemes.contains("se"));
        assert!(phonemes.contains("ka"));
    }
}
