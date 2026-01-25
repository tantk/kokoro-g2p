//! German G2P (Grapheme-to-Phoneme) module
//!
//! German has relatively consistent orthography with some complexities:
//! - Umlauts (ä, ö, ü)
//! - Compound words
//! - ch varies by context (/x/ after back vowels, /ç/ after front vowels)
//! - Final devoicing

pub mod normalizer;

use crate::tokenizer;

/// German G2P processor
pub struct GermanG2P;

impl GermanG2P {
    pub fn new() -> Self {
        Self
    }

    pub fn text_to_phonemes(&self, text: &str) -> String {
        let normalized = normalizer::normalize(text);
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

    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        tokenizer::phonemes_to_tokens(&phonemes)
    }
}

impl Default for GermanG2P {
    fn default() -> Self {
        Self::new()
    }
}

pub fn text_to_tokens(text: &str) -> Vec<i64> {
    GermanG2P::new().text_to_tokens(text)
}

pub fn text_to_phonemes(text: &str) -> String {
    GermanG2P::new().text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

fn word_to_phonemes(word: &str) -> String {
    let word_lower = word.to_lowercase();
    let chars: Vec<char> = word_lower.chars().collect();
    let mut phonemes = String::new();
    let mut i = 0;

    // German stress typically on first syllable (with exceptions for prefixes)
    let total_syllables = count_syllables(&chars);
    let stress_pos = 1; // First syllable default
    let mut syllable_count = 0;
    let mut stressed = false;

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let next2 = chars.get(i + 2).copied();
        let prev = if i > 0 { chars.get(i - 1).copied() } else { None };

        // Add stress marker before first stressed vowel
        if is_vowel(c) {
            syllable_count += 1;
            if syllable_count == stress_pos && !stressed && total_syllables > 1 {
                phonemes.push('ˈ');
                stressed = true;
            }
        }

        // Trigraphs and digraphs
        match (c, next, next2) {
            // sch → /ʃ/
            ('s', Some('c'), Some('h')) => {
                phonemes.push('ʃ');
                i += 3;
                continue;
            }
            // tsch → /ʧ/
            ('t', Some('s'), Some('c')) if chars.get(i + 3) == Some(&'h') => {
                phonemes.push('ʧ');
                i += 4;
                continue;
            }
            _ => {}
        }

        match (c, next) {
            // ch → /ç/ after front vowels (e, i, ä, ö, ü) or consonants, /x/ after back vowels
            ('c', Some('h')) => {
                if matches!(prev, Some('a') | Some('o') | Some('u') | Some('ɑ')) {
                    phonemes.push('x'); // ach-Laut
                } else {
                    phonemes.push('ç'); // ich-Laut
                }
                i += 2;
                continue;
            }
            // ck → /k/
            ('c', Some('k')) => {
                phonemes.push('k');
                i += 2;
                continue;
            }
            // ie → /iː/ (long i)
            ('i', Some('e')) => {
                phonemes.push('i');
                phonemes.push('ː');
                i += 2;
                continue;
            }
            // ei → /aɪ/
            ('e', Some('i')) => {
                phonemes.push('a');
                phonemes.push('ɪ');
                i += 2;
                continue;
            }
            // eu, äu → /ɔʏ/
            ('e', Some('u')) | ('ä', Some('u')) => {
                phonemes.push('ɔ');
                phonemes.push('ʏ');
                i += 2;
                continue;
            }
            // au → /aʊ/
            ('a', Some('u')) => {
                phonemes.push('a');
                phonemes.push('ʊ');
                i += 2;
                continue;
            }
            // ng → /ŋ/
            ('n', Some('g')) => {
                phonemes.push('ŋ');
                i += 2;
                continue;
            }
            // pf → /pf/
            ('p', Some('f')) => {
                phonemes.push('p');
                phonemes.push('f');
                i += 2;
                continue;
            }
            // qu → /kv/
            ('q', Some('u')) => {
                phonemes.push('k');
                phonemes.push('v');
                i += 2;
                continue;
            }
            // sp at start → /ʃp/
            ('s', Some('p')) if i == 0 => {
                phonemes.push('ʃ');
                phonemes.push('p');
                i += 2;
                continue;
            }
            // st at start → /ʃt/
            ('s', Some('t')) if i == 0 => {
                phonemes.push('ʃ');
                phonemes.push('t');
                i += 2;
                continue;
            }
            // tz → /ts/
            ('t', Some('z')) => {
                phonemes.push('t');
                phonemes.push('s');
                i += 2;
                continue;
            }
            // doubled vowels for length
            (v, Some(v2)) if v == v2 && is_vowel(v) => {
                phonemes.push(vowel_to_phoneme(v));
                phonemes.push('ː');
                i += 2;
                continue;
            }
            _ => {}
        }

        // Single characters
        match c {
            // Vowels
            'a' => phonemes.push('a'),
            'e' => phonemes.push('ɛ'),
            'i' => phonemes.push('ɪ'),
            'o' => phonemes.push('ɔ'),
            'u' => phonemes.push('ʊ'),
            'ä' => phonemes.push('ɛ'),
            'ö' => phonemes.push('ø'),
            'ü' => phonemes.push('y'),
            'y' => phonemes.push('y'),

            // Consonants
            'b' => {
                // Final devoicing
                if i == chars.len() - 1 {
                    phonemes.push('p');
                } else {
                    phonemes.push('b');
                }
            }
            'c' => phonemes.push('k'), // Before a, o, u
            'd' => {
                // Final devoicing
                if i == chars.len() - 1 {
                    phonemes.push('t');
                } else {
                    phonemes.push('d');
                }
            }
            'f' => phonemes.push('f'),
            'g' => {
                // Final devoicing
                if i == chars.len() - 1 {
                    phonemes.push('k');
                } else {
                    phonemes.push('ɡ');
                }
            }
            'h' => {
                // h is silent after vowels (lengthening marker)
                if !prev.map_or(false, is_vowel) {
                    phonemes.push('h');
                }
            }
            'j' => phonemes.push('j'),
            'k' => phonemes.push('k'),
            'l' => phonemes.push('l'),
            'm' => phonemes.push('m'),
            'n' => phonemes.push('n'),
            'p' => phonemes.push('p'),
            'r' => phonemes.push('ʁ'), // Uvular fricative (standard German)
            's' => {
                // s between vowels or before vowels at word start is voiced
                if next.map_or(false, is_vowel) && (i == 0 || prev.map_or(false, is_vowel)) {
                    phonemes.push('z');
                } else {
                    phonemes.push('s');
                }
            }
            'ß' => phonemes.push('s'), // Always voiceless
            't' => phonemes.push('t'),
            'v' => phonemes.push('f'), // Usually /f/ in native words
            'w' => phonemes.push('v'),
            'x' => {
                phonemes.push('k');
                phonemes.push('s');
            }
            'z' => {
                phonemes.push('t');
                phonemes.push('s');
            }

            '.' | ',' | '!' | '?' | ';' | ':' => phonemes.push(c),
            _ => {}
        }

        i += 1;
    }

    phonemes
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü' | 'y')
}

fn vowel_to_phoneme(c: char) -> char {
    match c {
        'a' => 'a',
        'e' => 'e',
        'i' => 'i',
        'o' => 'o',
        'u' => 'u',
        'ä' => 'ɛ',
        'ö' => 'ø',
        'ü' => 'y',
        _ => c,
    }
}

fn count_syllables(chars: &[char]) -> usize {
    chars.iter().filter(|&&c| is_vowel(c)).count().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = GermanG2P::new();
        let phonemes = g2p.text_to_phonemes("hallo");
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_umlauts() {
        let phonemes = word_to_phonemes("schön");
        assert!(phonemes.contains('ø'));

        let phonemes = word_to_phonemes("für");
        assert!(phonemes.contains('y'));
    }

    #[test]
    fn test_sch() {
        let phonemes = word_to_phonemes("schule");
        assert!(phonemes.contains('ʃ'));
    }

    #[test]
    fn test_ch_variations() {
        // ich-Laut after front vowel
        let phonemes = word_to_phonemes("ich");
        assert!(phonemes.contains('ç'));

        // ach-Laut after back vowel
        let phonemes = word_to_phonemes("ach");
        assert!(phonemes.contains('x'));
    }

    #[test]
    fn test_diphthongs() {
        let phonemes = word_to_phonemes("mein");
        assert!(phonemes.contains('a') && phonemes.contains('ɪ'));
    }

    #[test]
    fn test_final_devoicing() {
        let phonemes = word_to_phonemes("tag");
        // Final g should become k
        assert!(phonemes.ends_with('k'));
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("guten tag");
        assert!(tokens.len() > 2);
    }
}
