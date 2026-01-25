//! G2P (Grapheme-to-Phoneme) Engine
//!
//! Main conversion pipeline that transforms English text into phoneme sequences
//! suitable for the Kokoro TTS model.

use crate::lexicon::LexiconRef;
use crate::preprocessor::{preprocess, tokenize};
use crate::tokenizer::phonemes_to_tokens;
pub use crate::tokenizer::{MAX_TOKENS, PAD_TOKEN};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

/// Characters that should be passed through as phonemes
static PASSTHROUGH_PUNCT: Lazy<HashSet<char>> = Lazy::new(|| {
    let chars = ".!?,;:—…\"()\u{201C}\u{201D}";  // " and " are U+201C and U+201D
    chars.chars().collect()
});

/// G2P Engine for converting text to phonemes
pub struct G2P {
    lexicon: LexiconRef,
    #[allow(dead_code)] // Reserved for future dialect-specific features
    british: bool,
    unk_marker: String,
}

impl Default for G2P {
    fn default() -> Self {
        Self::new(false)
    }
}

impl G2P {
    /// Create a new G2P engine
    ///
    /// # Arguments
    /// * `british` - Use British English pronunciation if true
    pub fn new(british: bool) -> Self {
        Self {
            lexicon: crate::lexicon::Lexicon::new_static(british),
            british,
            unk_marker: "❓".to_string(),
        }
    }

    /// Set the unknown word marker
    pub fn set_unk_marker(&mut self, marker: &str) {
        self.unk_marker = marker.to_string();
    }

    /// Check if this G2P engine is configured for British English
    pub fn is_british(&self) -> bool {
        self.british
    }

    /// Convert text to phoneme string
    pub fn text_to_phonemes(&self, text: &str) -> String {
        // Preprocess the text (numbers, abbreviations, etc.)
        let preprocessed = preprocess(text);

        // Tokenize into words and punctuation
        let tokens = tokenize(&preprocessed);

        // Convert each token to phonemes
        let mut result = String::new();
        let mut prev_was_word = false;

        for token in tokens {
            if token.is_punct {
                // Handle punctuation
                let punct = token.text.chars().next().unwrap_or(' ');
                if PASSTHROUGH_PUNCT.contains(&punct) {
                    // Normalize quotes
                    let normalized = match punct {
                        '"' | '\u{201C}' | '\u{201D}' => '"',
                        '\u{2018}' | '\u{2019}' => '\'',
                        _ => punct,
                    };
                    result.push(normalized);
                }
                prev_was_word = false;
            } else {
                // Add space between words
                if prev_was_word && !result.is_empty() && !result.ends_with(' ') {
                    result.push(' ');
                }

                // Convert word to phonemes
                let phonemes = self.word_to_phonemes(&token.text, None);
                result.push_str(&phonemes);
                prev_was_word = true;
            }

            // Handle whitespace
            if !token.whitespace.is_empty() && prev_was_word {
                // Whitespace after word handled by next iteration
            }
        }

        // Clean up the result
        self.clean_phonemes(&result)
    }

    /// Convert a single word to phonemes
    pub fn word_to_phonemes(&self, word: &str, tag: Option<&str>) -> String {
        // Skip empty words
        if word.is_empty() {
            return String::new();
        }

        // Handle contractions
        if let Some(phonemes) = self.handle_contraction(word) {
            return phonemes;
        }

        // Try dictionary lookup
        if let Some((phonemes, _rating)) = self.lexicon.get_word(word, tag) {
            return phonemes;
        }

        // Try lowercase
        let lower = word.to_lowercase();
        if lower != word {
            if let Some((phonemes, _rating)) = self.lexicon.get_word(&lower, tag) {
                return phonemes;
            }
        }

        // Handle compound words (split by hyphen or camelCase)
        if word.contains('-') || has_internal_caps(word) {
            if let Some(phonemes) = self.handle_compound(word, tag) {
                return phonemes;
            }
        }

        // Handle all-caps acronyms
        if word.len() <= 5 && word.chars().all(|c| c.is_ascii_uppercase()) {
            return self.spell_out(word);
        }

        // Fallback: try letter-by-letter for short words
        if word.len() <= 3 {
            return self.spell_out(word);
        }

        // Ultimate fallback: use unknown marker
        log::warn!("Unknown word: {}", word);
        self.unk_marker.clone()
    }

    /// Handle contractions like "don't", "I'm", etc.
    fn handle_contraction(&self, word: &str) -> Option<String> {
        // Common contractions
        let contractions: &[(&str, &str)] = &[
            ("can't", "kˈænt"),
            ("won't", "wˈOnt"),
            ("don't", "dˈOnt"),
            ("didn't", "dˈɪdᵊnt"),
            ("doesn't", "dˈʌzᵊnt"),
            ("couldn't", "kˈʊdᵊnt"),
            ("wouldn't", "wˈʊdᵊnt"),
            ("shouldn't", "ʃˈʊdᵊnt"),
            ("isn't", "ˈɪzᵊnt"),
            ("aren't", "ˈɑɹᵊnt"),
            ("wasn't", "wˈɑzᵊnt"),
            ("weren't", "wˈɜɹᵊnt"),
            ("haven't", "hˈævᵊnt"),
            ("hasn't", "hˈæzᵊnt"),
            ("hadn't", "hˈædᵊnt"),
            ("I'm", "ˌIm"),
            ("I've", "ˌIv"),
            ("I'll", "ˌIl"),
            ("I'd", "ˌId"),
            ("you're", "jˈʊɹ"),
            ("you've", "jˈuv"),
            ("you'll", "jˈul"),
            ("you'd", "jˈud"),
            ("he's", "hˈiz"),
            ("she's", "ʃˈiz"),
            ("it's", "ˈɪts"),
            ("we're", "wˈɪɹ"),
            ("we've", "wˈiv"),
            ("we'll", "wˈil"),
            ("we'd", "wˈid"),
            ("they're", "ðˈɛɹ"),
            ("they've", "ðˈAv"),
            ("they'll", "ðˈAl"),
            ("they'd", "ðˈAd"),
            ("that's", "ðˈæts"),
            ("what's", "wˈʌts"),
            ("there's", "ðˈɛɹz"),
            ("here's", "hˈɪɹz"),
            ("let's", "lˈɛts"),
        ];

        let word_lower = word.to_lowercase();
        for (contraction, phonemes) in contractions {
            if &word_lower == *contraction {
                return Some(phonemes.to_string());
            }
        }

        // Handle 's, 're, 've, 'll, 'd suffixes dynamically
        if word.ends_with("'s") || word.ends_with("'s") {
            let base = &word[..word.len() - 2];
            if let Some((base_phonemes, _)) = self.lexicon.get_word(base, None) {
                let last = base_phonemes.chars().last().unwrap_or(' ');
                let suffix = if "szʃʒʧʤ".contains(last) {
                    "ᵻz"
                } else if "ptkfθ".contains(last) {
                    "s"
                } else {
                    "z"
                };
                return Some(format!("{}{}", base_phonemes, suffix));
            }
        }

        None
    }

    /// Handle compound words
    fn handle_compound(&self, word: &str, tag: Option<&str>) -> Option<String> {
        // Split by hyphens
        if word.contains('-') {
            let parts: Vec<&str> = word.split('-').collect();
            let phoneme_parts: Vec<String> = parts
                .iter()
                .map(|p| self.word_to_phonemes(p, tag))
                .collect();

            if phoneme_parts.iter().all(|p| !p.contains(&self.unk_marker)) {
                return Some(phoneme_parts.join(" "));
            }
        }

        // Split by camelCase
        let parts = split_camel_case(word);
        if parts.len() > 1 {
            let phoneme_parts: Vec<String> = parts
                .iter()
                .map(|p| self.word_to_phonemes(p, tag))
                .collect();

            if phoneme_parts.iter().all(|p| !p.contains(&self.unk_marker)) {
                return Some(phoneme_parts.join(" "));
            }
        }

        None
    }

    /// Spell out a word letter by letter
    fn spell_out(&self, word: &str) -> String {
        let letter_phonemes: Vec<&str> = word
            .chars()
            .filter_map(|c| {
                match c.to_ascii_uppercase() {
                    'A' => Some("ˈA"),
                    'B' => Some("bˈi"),
                    'C' => Some("sˈi"),
                    'D' => Some("dˈi"),
                    'E' => Some("ˈi"),
                    'F' => Some("ˈɛf"),
                    'G' => Some("ʤˈi"),
                    'H' => Some("ˈAʧ"),
                    'I' => Some("ˈI"),
                    'J' => Some("ʤˈA"),
                    'K' => Some("kˈA"),
                    'L' => Some("ˈɛl"),
                    'M' => Some("ˈɛm"),
                    'N' => Some("ˈɛn"),
                    'O' => Some("ˈO"),
                    'P' => Some("pˈi"),
                    'Q' => Some("kjˈu"),
                    'R' => Some("ˈɑɹ"),
                    'S' => Some("ˈɛs"),
                    'T' => Some("tˈi"),
                    'U' => Some("jˈu"),
                    'V' => Some("vˈi"),
                    'W' => Some("dˈʌbᵊljˌu"),
                    'X' => Some("ˈɛks"),
                    'Y' => Some("wˈI"),
                    'Z' => Some("zˈi"),
                    _ => None,
                }
            })
            .collect();

        if letter_phonemes.is_empty() {
            self.unk_marker.clone()
        } else {
            letter_phonemes.join(" ")
        }
    }

    /// Clean up phoneme string
    fn clean_phonemes(&self, phonemes: &str) -> String {
        // Remove multiple spaces
        let re = Regex::new(r"\s+").unwrap();
        let cleaned = re.replace_all(phonemes, " ");

        // Trim
        cleaned.trim().to_string()
    }
}

/// Check if word has internal capital letters (camelCase)
fn has_internal_caps(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    for i in 1..chars.len() {
        if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
            return true;
        }
    }
    false
}

/// Split camelCase or PascalCase words
fn split_camel_case(word: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();

    for c in word.chars() {
        if c.is_uppercase() && !current.is_empty() {
            parts.push(current);
            current = String::new();
        }
        current.push(c);
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

/// Main entry point: Convert text to token IDs
pub fn text_to_tokens(text: &str, language: &str) -> Vec<i64> {
    let british = language.to_lowercase().contains("gb")
        || language.to_lowercase().contains("british")
        || language.to_lowercase() == "en-gb";

    let g2p = G2P::new(british);
    let phonemes = g2p.text_to_phonemes(text);

    phonemes_to_tokens(&phonemes)
}

/// Convert text to phoneme string (for debugging)
pub fn text_to_phoneme_string(text: &str, language: &str) -> String {
    let british = language.to_lowercase().contains("gb")
        || language.to_lowercase().contains("british")
        || language.to_lowercase() == "en-gb";

    let g2p = G2P::new(british);
    g2p.text_to_phonemes(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_tokens() {
        let tokens = text_to_tokens("Hello, world!", "en");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    fn test_phoneme_conversion() {
        let g2p = G2P::new(false);
        let phonemes = g2p.text_to_phonemes("hello");
        assert!(!phonemes.is_empty());
        assert!(!phonemes.contains("❓"));
    }

    #[test]
    fn test_contractions() {
        let g2p = G2P::new(false);
        let phonemes = g2p.text_to_phonemes("don't");
        assert!(!phonemes.contains("❓"));
    }

    #[test]
    fn test_numbers() {
        let tokens = text_to_tokens("I have 3 apples.", "en");
        assert!(tokens.len() > 5);
    }

    #[test]
    fn test_punctuation() {
        let g2p = G2P::new(false);
        let phonemes = g2p.text_to_phonemes("Hello! How are you?");
        assert!(phonemes.contains('!'));
        assert!(phonemes.contains('?'));
    }

    #[test]
    fn test_british() {
        let g2p_us = G2P::new(false);
        let g2p_gb = G2P::new(true);

        // Some words should have different pronunciations
        let us = g2p_us.text_to_phonemes("water");
        let gb = g2p_gb.text_to_phonemes("water");

        // Both should produce valid phonemes
        assert!(!us.is_empty());
        assert!(!gb.is_empty());
    }

    #[test]
    fn test_max_length() {
        let long_text = "word ".repeat(200);
        let tokens = text_to_tokens(&long_text, "en");
        assert!(tokens.len() <= MAX_TOKENS + 2);
    }

    #[test]
    fn test_acronyms() {
        let g2p = G2P::new(false);
        let phonemes = g2p.text_to_phonemes("USA");
        assert!(!phonemes.contains("❓"));
    }

    #[test]
    fn test_compound_words() {
        let g2p = G2P::new(false);

        // Hyphenated
        let phonemes = g2p.text_to_phonemes("ice-cream");
        assert!(!phonemes.contains("❓"));

        // CamelCase
        let _phonemes = g2p.text_to_phonemes("JavaScript");
        // May contain unknown but parts should be recognized
    }

    #[test]
    fn test_special_words() {
        let g2p = G2P::new(false);

        // Common words that need special handling
        let cases = ["the", "a", "an", "to", "I", "you", "we", "they"];
        for word in cases {
            let phonemes = g2p.text_to_phonemes(word);
            assert!(!phonemes.is_empty(), "Failed for word: {}", word);
            assert!(!phonemes.contains("❓"), "Unknown marker for word: {}", word);
        }
    }
}
