//! Italian G2P (Grapheme-to-Phoneme) module
//!
//! Italian has very regular orthography with consistent grapheme-phoneme correspondence.
//! The main challenges are stress placement (not always marked) and distinguishing
//! open/closed e and o.

pub mod normalizer;

use crate::tokenizer;

/// Italian G2P processor
pub struct ItalianG2P;

impl ItalianG2P {
    /// Create a new Italian G2P processor
    pub fn new() -> Self {
        Self
    }

    /// Convert Italian text to phoneme string (IPA-based)
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

    /// Convert Italian text to token IDs
    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        tokenizer::phonemes_to_tokens(&phonemes)
    }
}

impl Default for ItalianG2P {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert Italian text to token IDs (convenience function)
pub fn text_to_tokens(text: &str) -> Vec<i64> {
    let g2p = ItalianG2P::new();
    g2p.text_to_tokens(text)
}

/// Convert Italian text to phoneme string (convenience function)
pub fn text_to_phonemes(text: &str) -> String {
    let g2p = ItalianG2P::new();
    g2p.text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

/// Convert a single Italian word to IPA phonemes
fn word_to_phonemes(word: &str) -> String {
    let word_lower = word.to_lowercase();
    let chars: Vec<char> = word_lower.chars().collect();
    let mut phonemes = String::new();
    let mut i = 0;

    // Italian stress is typically on penultimate syllable (with exceptions)
    let total_syllables = count_syllables(&chars);
    let stress_pos = find_stress_position(&chars, total_syllables);
    let mut syllable_count = 0;

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

        // Trigraphs and digraphs
        match (c, next, next2) {
            // gli → /ʎ/ (before vowel) or /ɡli/ (word-final or before consonant)
            ('g', Some('l'), Some('i')) => {
                if chars.get(i + 3).map_or(false, |&c| is_vowel(c)) {
                    phonemes.push('ʎ');
                    i += 3;
                    continue;
                } else {
                    phonemes.push('ʎ');
                    phonemes.push('i');
                    i += 3;
                    continue;
                }
            }
            // gn → /ɲ/
            ('g', Some('n'), _) => {
                phonemes.push('ɲ');
                i += 2;
                continue;
            }
            // sc + e/i → /ʃ/
            ('s', Some('c'), Some(v)) if matches!(v, 'e' | 'i' | 'é' | 'í' | 'è' | 'ì') => {
                phonemes.push('ʃ');
                i += 2;
                continue;
            }
            _ => {}
        }

        // Digraphs
        match (c, next) {
            // ch → /k/ (before e, i)
            ('c', Some('h')) => {
                phonemes.push('k');
                i += 2;
                continue;
            }
            // gh → /ɡ/ (before e, i)
            ('g', Some('h')) => {
                phonemes.push('ɡ');
                i += 2;
                continue;
            }
            // Double consonants (gemination) - represent with length mark
            (a, Some(b)) if a == b && is_consonant(a) => {
                let phon = single_consonant_to_phoneme(a);
                if !phon.is_empty() {
                    phonemes.push_str(&phon);
                    phonemes.push('ː'); // Gemination marker
                }
                i += 2;
                continue;
            }
            _ => {}
        }

        // Single character conversions
        match c {
            // Vowels - Italian has 7 vowels in stressed position
            'a' | 'à' => phonemes.push('a'),
            'e' => phonemes.push('e'), // Could be /e/ or /ɛ/ - we use /e/ by default
            'é' => phonemes.push('e'), // Closed e
            'è' => phonemes.push('ɛ'), // Open e
            'i' | 'ì' | 'í' => phonemes.push('i'),
            'o' => phonemes.push('o'), // Could be /o/ or /ɔ/ - we use /o/ by default
            'ó' => phonemes.push('o'), // Closed o
            'ò' => phonemes.push('ɔ'), // Open o
            'u' | 'ù' | 'ú' => phonemes.push('u'),

            // Consonants
            'b' => phonemes.push('b'),
            'c' => {
                // c + e/i → /ʧ/, otherwise /k/
                if matches!(next, Some('e') | Some('i') | Some('é') | Some('í') | Some('è') | Some('ì')) {
                    phonemes.push('ʧ');
                } else {
                    phonemes.push('k');
                }
            }
            'd' => phonemes.push('d'),
            'f' => phonemes.push('f'),
            'g' => {
                // g + e/i → /ʤ/, otherwise /ɡ/
                if matches!(next, Some('e') | Some('i') | Some('é') | Some('í') | Some('è') | Some('ì')) {
                    phonemes.push('ʤ');
                } else {
                    phonemes.push('ɡ');
                }
            }
            'h' => {} // Silent in Italian
            'j' => phonemes.push('j'), // Rare, in loanwords
            'k' => phonemes.push('k'), // Rare, in loanwords
            'l' => phonemes.push('l'),
            'm' => phonemes.push('m'),
            'n' => phonemes.push('n'),
            'p' => phonemes.push('p'),
            'q' => phonemes.push('k'), // Always followed by 'u'
            'r' => phonemes.push('r'), // Trilled
            's' => {
                // s between vowels is often voiced /z/
                let prev = if i > 0 { chars.get(i - 1).copied() } else { None };
                if prev.map_or(false, is_vowel) && next.map_or(false, is_vowel) {
                    phonemes.push('z');
                } else {
                    phonemes.push('s');
                }
            }
            't' => phonemes.push('t'),
            'v' => phonemes.push('v'),
            'w' => phonemes.push('w'), // Loanwords
            'x' => {
                phonemes.push('k');
                phonemes.push('s');
            }
            'y' => phonemes.push('i'), // Loanwords
            'z' => {
                // z can be /ts/ or /dz/ - use /ts/ by default (more common)
                phonemes.push('ʦ');
            }

            // Punctuation passthrough
            '.' | ',' | '!' | '?' | ';' | ':' => phonemes.push(c),

            _ => {}
        }

        i += 1;
    }

    phonemes
}

fn single_consonant_to_phoneme(c: char) -> String {
    match c {
        'b' => "b",
        'c' => "k",
        'd' => "d",
        'f' => "f",
        'g' => "ɡ",
        'l' => "l",
        'm' => "m",
        'n' => "n",
        'p' => "p",
        'r' => "r",
        's' => "s",
        't' => "t",
        'v' => "v",
        'z' => "ʦ",
        _ => "",
    }.to_string()
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'à' | 'è' | 'é' | 'ì' | 'í' | 'ò' | 'ó' | 'ù' | 'ú')
}

fn is_consonant(c: char) -> bool {
    c.is_alphabetic() && !is_vowel(c)
}

fn count_syllables(chars: &[char]) -> usize {
    chars.iter().filter(|&&c| is_vowel(c)).count().max(1)
}

/// Find which syllable should be stressed (1-indexed)
/// Italian stress rules:
/// 1. Accented vowel always indicates stress
/// 2. Default: penultimate syllable
fn find_stress_position(chars: &[char], total: usize) -> usize {
    if total <= 1 {
        return 1;
    }

    // Check for written accent
    let mut current_syllable = 0;
    for &c in chars {
        if is_vowel(c) {
            current_syllable += 1;
        }
        if matches!(c, 'à' | 'è' | 'é' | 'ì' | 'í' | 'ò' | 'ó' | 'ù' | 'ú') {
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
        let g2p = ItalianG2P::new();
        let phonemes = g2p.text_to_phonemes("ciao");
        assert!(!phonemes.is_empty());
        assert!(phonemes.contains('ʧ')); // c before i = /ʧ/
    }

    #[test]
    fn test_ch_digraph() {
        let phonemes = word_to_phonemes("che");
        assert!(phonemes.contains('k'));
    }

    #[test]
    fn test_gn_digraph() {
        let phonemes = word_to_phonemes("gnocchi");
        assert!(phonemes.contains('ɲ'));
    }

    #[test]
    fn test_gli_trigraph() {
        let phonemes = word_to_phonemes("figlio");
        assert!(phonemes.contains('ʎ'));
    }

    #[test]
    fn test_sc_digraph() {
        let phonemes = word_to_phonemes("scena");
        assert!(phonemes.contains('ʃ'));
    }

    #[test]
    fn test_gemination() {
        let phonemes = word_to_phonemes("bello");
        // Should have length mark for double l
        assert!(phonemes.contains('ː'));
    }

    #[test]
    fn test_open_vowels() {
        // è = open e
        let phonemes = word_to_phonemes("caffè");
        assert!(phonemes.contains('ɛ'));
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("ciao mondo");
        assert!(tokens.len() > 2);
    }
}
