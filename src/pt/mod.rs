//! Portuguese G2P (Grapheme-to-Phoneme) module
//!
//! Portuguese has relatively regular orthography with some complexities:
//! - Nasal vowels (ã, õ, ~)
//! - Open/closed vowels (not always marked)
//! - Brazilian vs European differences (we default to Brazilian)

pub mod normalizer;

use crate::tokenizer;

/// Portuguese G2P processor
pub struct PortugueseG2P {
    brazilian: bool,
}

impl PortugueseG2P {
    pub fn new() -> Self {
        Self { brazilian: true }
    }

    pub fn new_european() -> Self {
        Self { brazilian: false }
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
                let phonemes = self.word_to_phonemes(word);
                result.push_str(&phonemes);
            }
        }
        result
    }

    fn word_to_phonemes(&self, word: &str) -> String {
        let word_lower = word.to_lowercase();
        let chars: Vec<char> = word_lower.chars().collect();
        let mut phonemes = String::new();
        let mut i = 0;

        let total_syllables = count_syllables(&chars);
        let stress_pos = find_stress_position(&chars, total_syllables);
        let mut syllable_count = 0;

        while i < chars.len() {
            let c = chars[i];
            let next = chars.get(i + 1).copied();
            let next2 = chars.get(i + 2).copied();

            if is_vowel(c) {
                syllable_count += 1;
                if syllable_count == stress_pos && total_syllables > 1 {
                    phonemes.push('ˈ');
                }
            }

            // Digraphs
            match (c, next, next2) {
                // lh → /ʎ/
                ('l', Some('h'), _) => {
                    phonemes.push('ʎ');
                    i += 2;
                    continue;
                }
                // nh → /ɲ/
                ('n', Some('h'), _) => {
                    phonemes.push('ɲ');
                    i += 2;
                    continue;
                }
                // ch → /ʃ/
                ('c', Some('h'), _) => {
                    phonemes.push('ʃ');
                    i += 2;
                    continue;
                }
                // rr → /ʁ/ (strong r)
                ('r', Some('r'), _) => {
                    phonemes.push('ʁ');
                    i += 2;
                    continue;
                }
                // ss → /s/
                ('s', Some('s'), _) => {
                    phonemes.push('s');
                    i += 2;
                    continue;
                }
                // qu → /k/ before e, i
                ('q', Some('u'), Some(v)) if matches!(v, 'e' | 'i' | 'é' | 'í') => {
                    phonemes.push('k');
                    i += 2;
                    continue;
                }
                // gu → /g/ before e, i
                ('g', Some('u'), Some(v)) if matches!(v, 'e' | 'i' | 'é' | 'í') => {
                    phonemes.push('ɡ');
                    i += 2;
                    continue;
                }
                _ => {}
            }

            match (c, next) {
                // ão → /ɐ̃w̃/ (nasal diphthong)
                ('ã', Some('o')) => {
                    phonemes.push('ɐ');
                    phonemes.push('\u{0303}'); // combining tilde
                    phonemes.push('w');
                    i += 2;
                    continue;
                }
                // õe → /õj̃/
                ('õ', Some('e')) => {
                    phonemes.push('o');
                    phonemes.push('\u{0303}');
                    phonemes.push('j');
                    i += 2;
                    continue;
                }
                _ => {}
            }

            // Single characters
            match c {
                // Oral vowels
                'a' => phonemes.push('a'),
                'á' => phonemes.push('a'),
                'â' => phonemes.push('ɐ'),
                'e' => {
                    // Unstressed e often reduces in Brazilian Portuguese
                    if syllable_count != stress_pos && self.brazilian {
                        phonemes.push('i');
                    } else {
                        phonemes.push('e');
                    }
                }
                'é' => phonemes.push('ɛ'),
                'ê' => phonemes.push('e'),
                'i' | 'í' => phonemes.push('i'),
                'o' => {
                    // Unstressed o often reduces in Brazilian Portuguese
                    if syllable_count != stress_pos && self.brazilian {
                        phonemes.push('u');
                    } else {
                        phonemes.push('o');
                    }
                }
                'ó' => phonemes.push('ɔ'),
                'ô' => phonemes.push('o'),
                'u' | 'ú' => phonemes.push('u'),

                // Nasal vowels
                'ã' => {
                    phonemes.push('ɐ');
                    phonemes.push('\u{0303}');
                }
                'õ' => {
                    phonemes.push('o');
                    phonemes.push('\u{0303}');
                }

                // Consonants
                'b' => phonemes.push('b'),
                'c' => {
                    if matches!(next, Some('e') | Some('i') | Some('é') | Some('í')) {
                        phonemes.push('s');
                    } else {
                        phonemes.push('k');
                    }
                }
                'ç' => phonemes.push('s'),
                'd' => {
                    // In Brazilian Portuguese, d before i becomes /dʒ/
                    if self.brazilian && matches!(next, Some('i') | Some('í')) {
                        phonemes.push('ʤ');
                    } else {
                        phonemes.push('d');
                    }
                }
                'f' => phonemes.push('f'),
                'g' => {
                    if matches!(next, Some('e') | Some('i') | Some('é') | Some('í')) {
                        phonemes.push('ʒ');
                    } else {
                        phonemes.push('ɡ');
                    }
                }
                'h' => {} // Silent
                'j' => phonemes.push('ʒ'),
                'k' => phonemes.push('k'),
                'l' => {
                    // Final l often vocalizes to /w/ in Brazilian Portuguese
                    if self.brazilian && (i == chars.len() - 1 || !is_vowel(next.unwrap_or(' '))) {
                        phonemes.push('w');
                    } else {
                        phonemes.push('l');
                    }
                }
                'm' => {
                    // Final m nasalizes preceding vowel
                    if i == chars.len() - 1 {
                        // Already handled by nasal context
                    } else {
                        phonemes.push('m');
                    }
                }
                'n' => {
                    if i == chars.len() - 1 {
                        // Final n nasalizes preceding vowel
                    } else {
                        phonemes.push('n');
                    }
                }
                'p' => phonemes.push('p'),
                'q' => phonemes.push('k'),
                'r' => {
                    // Initial r or after n/l → strong /ʁ/
                    if i == 0 {
                        phonemes.push('ʁ');
                    } else {
                        phonemes.push('ɾ'); // Tap
                    }
                }
                's' => {
                    // s between vowels is voiced
                    let prev = if i > 0 { chars.get(i - 1).copied() } else { None };
                    if prev.map_or(false, is_vowel) && next.map_or(false, is_vowel) {
                        phonemes.push('z');
                    } else {
                        phonemes.push('s');
                    }
                }
                't' => {
                    // In Brazilian Portuguese, t before i becomes /tʃ/
                    if self.brazilian && matches!(next, Some('i') | Some('í')) {
                        phonemes.push('ʧ');
                    } else {
                        phonemes.push('t');
                    }
                }
                'v' => phonemes.push('v'),
                'w' => phonemes.push('w'),
                'x' => phonemes.push('ʃ'), // Most common
                'y' => phonemes.push('i'),
                'z' => phonemes.push('z'),

                '.' | ',' | '!' | '?' | ';' | ':' => phonemes.push(c),
                _ => {}
            }

            i += 1;
        }

        phonemes
    }

    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        tokenizer::phonemes_to_tokens(&phonemes)
    }
}

impl Default for PortugueseG2P {
    fn default() -> Self {
        Self::new()
    }
}

pub fn text_to_tokens(text: &str) -> Vec<i64> {
    PortugueseG2P::new().text_to_tokens(text)
}

pub fn text_to_phonemes(text: &str) -> String {
    PortugueseG2P::new().text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'á' | 'é' | 'í' | 'ó' | 'ú' | 'â' | 'ê' | 'ô' | 'ã' | 'õ')
}

fn count_syllables(chars: &[char]) -> usize {
    chars.iter().filter(|&&c| is_vowel(c)).count().max(1)
}

fn find_stress_position(chars: &[char], total: usize) -> usize {
    if total <= 1 {
        return 1;
    }

    // Check for accent marks
    let mut current_syllable = 0;
    for &c in chars {
        if is_vowel(c) {
            current_syllable += 1;
        }
        if matches!(c, 'á' | 'é' | 'í' | 'ó' | 'ú' | 'â' | 'ê' | 'ô' | 'ã' | 'õ') {
            return current_syllable;
        }
    }

    // Default: penultimate syllable
    total.saturating_sub(1).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = PortugueseG2P::new();
        let phonemes = g2p.text_to_phonemes("olá");
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_lh_nh() {
        let g2p = PortugueseG2P::new();
        let phonemes = g2p.text_to_phonemes("filho");
        assert!(phonemes.contains('ʎ'));

        let phonemes = g2p.text_to_phonemes("minha");
        assert!(phonemes.contains('ɲ'));
    }

    #[test]
    fn test_nasal_vowels() {
        let g2p = PortugueseG2P::new();
        let phonemes = g2p.text_to_phonemes("irmã");
        assert!(phonemes.contains('\u{0303}')); // Combining tilde
    }

    #[test]
    fn test_brazilian_palatalization() {
        let g2p = PortugueseG2P::new(); // Brazilian
        let phonemes = g2p.text_to_phonemes("dia");
        assert!(phonemes.contains('ʤ')); // d + i → dʒ

        let phonemes = g2p.text_to_phonemes("tia");
        assert!(phonemes.contains('ʧ')); // t + i → tʃ
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("bom dia");
        assert!(tokens.len() > 2);
    }
}
