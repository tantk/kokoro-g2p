//! Indonesian G2P (Grapheme-to-Phoneme) module
//!
//! Indonesian (Bahasa Indonesia) has very transparent orthography with near-perfect
//! grapheme-phoneme correspondence. The main challenge is the letter 'e' which can
//! represent three sounds: /e/, /ə/, or /ɛ/.

pub mod normalizer;

use crate::tokenizer;

/// Indonesian G2P processor
pub struct IndonesianG2P;

impl IndonesianG2P {
    /// Create a new Indonesian G2P processor
    pub fn new() -> Self {
        Self
    }

    /// Convert Indonesian text to phoneme string (IPA-based)
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

    /// Convert Indonesian text to token IDs
    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        tokenizer::phonemes_to_tokens(&phonemes)
    }
}

impl Default for IndonesianG2P {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Indonesian text to token IDs (convenience function)
pub fn text_to_tokens(text: &str) -> Vec<i64> {
    let g2p = IndonesianG2P::new();
    g2p.text_to_tokens(text)
}

/// Convert Indonesian text to phoneme string (convenience function)
pub fn text_to_phonemes(text: &str) -> String {
    let g2p = IndonesianG2P::new();
    g2p.text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

/// Convert a single Indonesian word to IPA phonemes
fn word_to_phonemes(word: &str) -> String {
    let word_lower = word.to_lowercase();
    let chars: Vec<char> = word_lower.chars().collect();
    let mut phonemes = String::new();
    let mut i = 0;

    // Indonesian stress is typically on penultimate syllable
    let total_syllables = count_syllables(&chars);
    let stress_pos = if total_syllables > 1 {
        total_syllables - 1
    } else {
        1
    };
    let mut syllable_count = 0;

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();

        // Add stress marker before stressed vowel
        if is_vowel(c) {
            syllable_count += 1;
            if syllable_count == stress_pos && total_syllables > 1 {
                phonemes.push('ˈ');
            }
        }

        // Digraphs
        match (c, next) {
            // ng → /ŋ/
            ('n', Some('g')) => {
                phonemes.push('ŋ');
                i += 2;
                continue;
            }
            // ny → /ɲ/
            ('n', Some('y')) => {
                phonemes.push('ɲ');
                i += 2;
                continue;
            }
            // sy → /ʃ/ (from Arabic loanwords)
            ('s', Some('y')) => {
                phonemes.push('ʃ');
                i += 2;
                continue;
            }
            // kh → /x/ (from Arabic loanwords)
            ('k', Some('h')) => {
                phonemes.push('x');
                i += 2;
                continue;
            }
            // gh → /ɣ/ (from Arabic loanwords)
            ('g', Some('h')) => {
                phonemes.push('ɣ');
                i += 2;
                continue;
            }
            _ => {}
        }

        // Single character conversions
        match c {
            // Vowels
            'a' => phonemes.push('a'),
            'i' => phonemes.push('i'),
            'u' => phonemes.push('u'),
            'o' => phonemes.push('o'),
            'e' => {
                // 'e' in Indonesian can be /e/, /ə/, or /ɛ/
                // Use schwa /ə/ in unstressed positions, /e/ in stressed
                // This is a simplification; proper handling requires a dictionary
                if syllable_count == stress_pos {
                    phonemes.push('e');
                } else {
                    phonemes.push('ə');
                }
            }

            // Consonants (mostly one-to-one)
            'b' => phonemes.push('b'),
            'c' => phonemes.push('ʧ'), // Indonesian 'c' is always /tʃ/
            'd' => phonemes.push('d'),
            'f' => phonemes.push('f'),
            'g' => phonemes.push('ɡ'),
            'h' => phonemes.push('h'),
            'j' => phonemes.push('ʤ'), // Indonesian 'j' is /dʒ/
            'k' => {
                // Final 'k' is often a glottal stop
                if i == chars.len() - 1 {
                    phonemes.push('ʔ');
                } else {
                    phonemes.push('k');
                }
            }
            'l' => phonemes.push('l'),
            'm' => phonemes.push('m'),
            'n' => phonemes.push('n'),
            'p' => phonemes.push('p'),
            'q' => phonemes.push('k'), // Rare, from loanwords
            'r' => phonemes.push('r'), // Trilled or tapped
            's' => phonemes.push('s'),
            't' => phonemes.push('t'),
            'v' => phonemes.push('f'), // Often pronounced as /f/
            'w' => phonemes.push('w'),
            'x' => {
                phonemes.push('k');
                phonemes.push('s');
            }
            'y' => phonemes.push('j'), // IPA /j/
            'z' => phonemes.push('z'),

            // Punctuation passthrough
            '.' | ',' | '!' | '?' | ';' | ':' => phonemes.push(c),

            _ => {}
        }

        i += 1;
    }

    phonemes
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn count_syllables(chars: &[char]) -> usize {
    chars.iter().filter(|&&c| is_vowel(c)).count().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = IndonesianG2P::new();
        let phonemes = g2p.text_to_phonemes("halo");
        assert!(!phonemes.is_empty());
        assert!(phonemes.contains('h'));
        assert!(phonemes.contains('a'));
        assert!(phonemes.contains('l'));
        assert!(phonemes.contains('o'));
    }

    #[test]
    fn test_ng_digraph() {
        let phonemes = word_to_phonemes("dengan");
        assert!(phonemes.contains('ŋ'));
    }

    #[test]
    fn test_ny_digraph() {
        let phonemes = word_to_phonemes("nyata");
        assert!(phonemes.contains('ɲ'));
    }

    #[test]
    fn test_c_sound() {
        // Indonesian 'c' is always /tʃ/
        let phonemes = word_to_phonemes("cinta");
        assert!(phonemes.contains('ʧ'));
    }

    #[test]
    fn test_final_k_glottal() {
        let phonemes = word_to_phonemes("tidak");
        assert!(phonemes.contains('ʔ'));
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("selamat pagi");
        assert!(tokens.len() > 2);
    }
}
