//! Turkish G2P (Grapheme-to-Phoneme) module
//!
//! Turkish has very regular orthography with near-phonetic spelling.
//! The main features are vowel harmony and special characters (ç, ğ, ı, ö, ş, ü).

pub mod normalizer;

use crate::tokenizer;

/// Turkish G2P processor
pub struct TurkishG2P;

impl TurkishG2P {
    /// Create a new Turkish G2P processor
    pub fn new() -> Self {
        Self
    }

    /// Convert Turkish text to phoneme string (IPA-based)
    pub fn text_to_phonemes(&self, text: &str) -> String {
        // Step 1: Normalize text (numbers, dates, currency)
        let normalized = normalizer::normalize(text);

        // Step 2: Convert to phonemes
        let mut result = String::new();
        let words: Vec<&str> = normalized.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                result.push(' ');
            }

            if is_punctuation(word) {
                result.push_str(word);
            } else {
                let phonemes = word_to_phonemes(word);
                result.push_str(&phonemes);
            }
        }

        result
    }

    /// Convert Turkish text to token IDs
    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        tokenizer::phonemes_to_tokens(&phonemes)
    }
}

impl Default for TurkishG2P {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Turkish text to token IDs (convenience function)
pub fn text_to_tokens(text: &str) -> Vec<i64> {
    let g2p = TurkishG2P::new();
    g2p.text_to_tokens(text)
}

/// Convert Turkish text to phoneme string (convenience function)
pub fn text_to_phonemes(text: &str) -> String {
    let g2p = TurkishG2P::new();
    g2p.text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

/// Convert a single Turkish word to IPA phonemes
fn word_to_phonemes(word: &str) -> String {
    let word_lower = word.to_lowercase();
    let chars: Vec<char> = word_lower.chars().collect();
    let mut phonemes = String::new();
    let mut i = 0;

    // Turkish stress is typically on the last syllable (with exceptions)
    let total_syllables = count_syllables(&chars);
    let stress_pos = total_syllables; // Last syllable
    let mut syllable_count = 0;

    while i < chars.len() {
        let c = chars[i];

        // Add stress marker before stressed vowel
        if is_vowel(c) {
            syllable_count += 1;
            if syllable_count == stress_pos && total_syllables > 1 {
                phonemes.push('ˈ');
            }
        }

        // Single character conversions (Turkish is very regular)
        match c {
            // Vowels - Turkish has 8 vowels with vowel harmony
            'a' => phonemes.push('a'),
            'e' => phonemes.push('e'),
            'ı' => phonemes.push('ɯ'), // Dotless i - close back unrounded
            'i' => phonemes.push('i'),
            'o' => phonemes.push('o'),
            'ö' => phonemes.push('ø'), // Close-mid front rounded
            'u' => phonemes.push('u'),
            'ü' => phonemes.push('y'), // Close front rounded (IPA y)

            // Consonants
            'b' => phonemes.push('b'),
            'c' => phonemes.push('ʤ'), // Turkish c is /dʒ/
            'ç' => phonemes.push('ʧ'), // Turkish ç is /tʃ/
            'd' => phonemes.push('d'),
            'f' => phonemes.push('f'),
            'g' => phonemes.push('ɡ'),
            'ğ' => {
                // Soft g - lengthens preceding vowel or is silent
                // Between back vowels: silent/lengthening
                // Between front vowels: weak /j/
                // We simplify by making it a length mark or /j/
                let prev = if i > 0 { chars.get(i - 1).copied() } else { None };
                if matches!(prev, Some('e') | Some('i') | Some('ö') | Some('ü')) {
                    phonemes.push('j');
                } else {
                    phonemes.push('ː'); // Lengthening
                }
            }
            'h' => phonemes.push('h'),
            'j' => phonemes.push('ʒ'), // Turkish j is /ʒ/
            'k' => phonemes.push('k'),
            'l' => phonemes.push('l'),
            'm' => phonemes.push('m'),
            'n' => phonemes.push('n'),
            'p' => phonemes.push('p'),
            'r' => phonemes.push('ɾ'), // Tap
            's' => phonemes.push('s'),
            'ş' => phonemes.push('ʃ'), // Turkish ş is /ʃ/
            't' => phonemes.push('t'),
            'v' => phonemes.push('v'),
            'y' => phonemes.push('j'), // Turkish y is /j/
            'z' => phonemes.push('z'),

            // Loanword letters
            'q' => phonemes.push('k'),
            'w' => phonemes.push('v'),
            'x' => {
                phonemes.push('k');
                phonemes.push('s');
            }

            // Punctuation passthrough
            '.' | ',' | '!' | '?' | ';' | ':' => phonemes.push(c),

            _ => {}
        }

        i += 1;
    }

    phonemes
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'ı' | 'i' | 'o' | 'ö' | 'u' | 'ü')
}

fn count_syllables(chars: &[char]) -> usize {
    chars.iter().filter(|&&c| is_vowel(c)).count().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = TurkishG2P::new();
        let phonemes = g2p.text_to_phonemes("merhaba");
        assert!(!phonemes.is_empty());
        assert!(phonemes.contains('m'));
        assert!(phonemes.contains('e'));
        assert!(phonemes.contains('ɾ'));
    }

    #[test]
    fn test_special_vowels() {
        // Test ı (dotless i)
        let phonemes = word_to_phonemes("kız");
        assert!(phonemes.contains('ɯ'));

        // Test ö
        let phonemes = word_to_phonemes("göz");
        assert!(phonemes.contains('ø'));

        // Test ü
        let phonemes = word_to_phonemes("gül");
        assert!(phonemes.contains('y'));
    }

    #[test]
    fn test_special_consonants() {
        // Test ç
        let phonemes = word_to_phonemes("çay");
        assert!(phonemes.contains('ʧ'));

        // Test ş
        let phonemes = word_to_phonemes("şeker");
        assert!(phonemes.contains('ʃ'));

        // Test c (= /dʒ/)
        let phonemes = word_to_phonemes("cami");
        assert!(phonemes.contains('ʤ'));
    }

    #[test]
    fn test_soft_g() {
        // ğ after front vowel
        let phonemes = word_to_phonemes("değil");
        assert!(phonemes.contains('j'));

        // ğ after back vowel - lengthening
        let phonemes = word_to_phonemes("dağ");
        assert!(phonemes.contains('ː'));
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("merhaba dünya");
        assert!(tokens.len() > 2);
    }
}
