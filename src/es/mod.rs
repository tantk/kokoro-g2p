//! Spanish G2P (Grapheme-to-Phoneme) module
//!
//! Spanish has very regular orthography with near one-to-one grapheme-phoneme correspondence.
//! This makes rule-based G2P highly accurate.

pub mod normalizer;

use crate::tokenizer;

/// Spanish G2P processor
pub struct SpanishG2P;

impl SpanishG2P {
    /// Create a new Spanish G2P processor
    pub fn new() -> Self {
        Self
    }

    /// Convert Spanish text to phoneme string (IPA-based)
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

            // Check if it's punctuation
            if is_punctuation(word) {
                result.push_str(word);
            } else {
                let phonemes = word_to_phonemes(word);
                result.push_str(&phonemes);
            }
        }

        result
    }

    /// Convert Spanish text to token IDs
    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        tokenizer::phonemes_to_tokens(&phonemes)
    }
}

impl Default for SpanishG2P {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Spanish text to token IDs (convenience function)
pub fn text_to_tokens(text: &str) -> Vec<i64> {
    let g2p = SpanishG2P::new();
    g2p.text_to_tokens(text)
}

/// Convert Spanish text to phoneme string (convenience function)
pub fn text_to_phonemes(text: &str) -> String {
    let g2p = SpanishG2P::new();
    g2p.text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

/// Convert a single Spanish word to IPA phonemes
fn word_to_phonemes(word: &str) -> String {
    let word_lower = word.to_lowercase();
    let chars: Vec<char> = word_lower.chars().collect();
    let mut phonemes = String::new();
    let mut i = 0;

    // Determine stress position
    let stress_pos = find_stress_position(&chars);
    let mut syllable_count = 0;
    let total_syllables = count_syllables(&chars);

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let next2 = chars.get(i + 2).copied();

        // Add stress marker before stressed vowel
        if is_vowel(c) {
            syllable_count += 1;
            if syllable_count == stress_pos && total_syllables > 1 {
                phonemes.push('ˈ');
            }
        }

        // Digraphs and special cases
        match (c, next) {
            // ch → /ʧ/
            ('c', Some('h')) => {
                phonemes.push('ʧ');
                i += 2;
                continue;
            }
            // ll → /ʎ/ or /ʝ/ (yeísmo - we use /ʝ/ which is more common)
            ('l', Some('l')) => {
                phonemes.push('ʝ');
                i += 2;
                continue;
            }
            // rr → /r/ (trilled)
            ('r', Some('r')) => {
                phonemes.push('r');
                i += 2;
                continue;
            }
            // qu → /k/ (before e, i)
            ('q', Some('u')) => {
                phonemes.push('k');
                i += 2;
                continue;
            }
            // gu → /g/ (before e, i) or /gw/ (before a, o)
            ('g', Some('u')) => {
                if matches!(next2, Some('e') | Some('i') | Some('é') | Some('í')) {
                    phonemes.push('ɡ');
                    i += 2;
                    continue;
                } else if matches!(next2, Some('a') | Some('o') | Some('á') | Some('ó')) {
                    phonemes.push('ɡ');
                    phonemes.push('w');
                    i += 2;
                    continue;
                }
            }
            _ => {}
        }

        // Single character conversions
        match c {
            // Vowels (Spanish has 5 pure vowels)
            'a' | 'á' => phonemes.push('a'),
            'e' | 'é' => phonemes.push('e'),
            'i' | 'í' | 'y' => {
                // 'y' is consonant at start of syllable, vowel otherwise
                if c == 'y' && (i == 0 || !is_vowel(chars.get(i.saturating_sub(1)).copied().unwrap_or(' '))) {
                    phonemes.push('ʝ');
                } else {
                    phonemes.push('i');
                }
            }
            'o' | 'ó' => phonemes.push('o'),
            'u' | 'ú' | 'ü' => phonemes.push('u'),

            // Consonants
            'b' | 'v' => {
                // /b/ at start of phrase or after nasal, /β/ elsewhere
                // Simplified: use /b/
                phonemes.push('b');
            }
            'c' => {
                // c + e/i → /θ/ (Castilian) or /s/ (Latin American)
                // We use /s/ for broader compatibility
                if matches!(next, Some('e') | Some('i') | Some('é') | Some('í')) {
                    phonemes.push('s');
                } else {
                    phonemes.push('k');
                }
            }
            'd' => phonemes.push('d'),
            'f' => phonemes.push('f'),
            'g' => {
                // g + e/i → /x/ (Spanish jota)
                if matches!(next, Some('e') | Some('i') | Some('é') | Some('í')) {
                    phonemes.push('x');
                } else {
                    phonemes.push('ɡ');
                }
            }
            'h' => {} // Silent in Spanish
            'j' => phonemes.push('x'), // Spanish jota /x/
            'k' => phonemes.push('k'),
            'l' => phonemes.push('l'),
            'm' => phonemes.push('m'),
            'n' => phonemes.push('n'),
            'ñ' => phonemes.push('ɲ'), // Palatal nasal
            'p' => phonemes.push('p'),
            'r' => {
                // Initial r or after n/l/s → trilled /r/, otherwise tap /ɾ/
                if i == 0 || matches!(chars.get(i.saturating_sub(1)), Some('n') | Some('l') | Some('s')) {
                    phonemes.push('r');
                } else {
                    phonemes.push('ɾ');
                }
            }
            's' => phonemes.push('s'),
            't' => phonemes.push('t'),
            'w' => phonemes.push('w'), // Loanwords
            'x' => {
                phonemes.push('k');
                phonemes.push('s');
            }
            'z' => phonemes.push('s'), // /θ/ in Castilian, /s/ in Latin American

            // Punctuation passthrough
            '.' | ',' | '!' | '?' | ';' | ':' => phonemes.push(c),

            // Unknown characters
            _ => {}
        }

        i += 1;
    }

    phonemes
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'á' | 'é' | 'í' | 'ó' | 'ú' | 'ü')
}

fn count_syllables(chars: &[char]) -> usize {
    chars.iter().filter(|&&c| is_vowel(c)).count().max(1)
}

/// Find which syllable should be stressed (1-indexed from the end)
/// Spanish stress rules:
/// 1. If word ends in vowel, n, or s → penultimate syllable
/// 2. If word ends in consonant (except n, s) → last syllable
/// 3. Written accent always indicates stress
fn find_stress_position(chars: &[char]) -> usize {
    let total = count_syllables(chars);
    if total <= 1 {
        return 1;
    }

    // Check for written accent (always determines stress)
    let mut accent_syllable = 0;
    let mut current_syllable = 0;
    for &c in chars {
        if is_vowel(c) {
            current_syllable += 1;
        }
        if matches!(c, 'á' | 'é' | 'í' | 'ó' | 'ú') {
            accent_syllable = current_syllable;
            break;
        }
    }

    if accent_syllable > 0 {
        return accent_syllable;
    }

    // Default rules based on final letter
    let last = chars.last().copied().unwrap_or(' ');
    if is_vowel(last) || last == 'n' || last == 's' {
        // Penultimate
        total.saturating_sub(1).max(1)
    } else {
        // Last syllable
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = SpanishG2P::new();
        let phonemes = g2p.text_to_phonemes("hola");
        assert!(!phonemes.is_empty());
        assert!(phonemes.contains('o'));
        assert!(phonemes.contains('l'));
        assert!(phonemes.contains('a'));
    }

    #[test]
    fn test_ch_digraph() {
        let phonemes = word_to_phonemes("mucho");
        assert!(phonemes.contains('ʧ'));
    }

    #[test]
    fn test_ll_digraph() {
        let phonemes = word_to_phonemes("llamar");
        assert!(phonemes.contains('ʝ'));
    }

    #[test]
    fn test_ene() {
        let phonemes = word_to_phonemes("niño");
        assert!(phonemes.contains('ɲ'));
    }

    #[test]
    fn test_stress_accent() {
        let phonemes = word_to_phonemes("café");
        assert!(phonemes.contains('ˈ'));
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("hola mundo");
        assert!(tokens.len() > 2);
    }
}
