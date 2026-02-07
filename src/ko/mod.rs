//! Korean G2P (Grapheme-to-Phoneme) module
//!
//! Korean uses Hangul, an alphabetic syllabary. Each syllable block contains:
//! - Initial consonant (choseong)
//! - Medial vowel (jungseong)
//! - Optional final consonant (jongseong)
//!
//! Key phonological rules:
//! - Liaison (연음): Final consonant moves to next syllable if it starts with ㅇ
//! - Nasalization (비음화): Stops become nasals before nasals
//! - Fortition (경음화): Certain consonant clusters become tense

pub mod normalizer;

use crate::tokenizer;

/// Korean G2P processor
pub struct KoreanG2P;

impl KoreanG2P {
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

impl Default for KoreanG2P {
    fn default() -> Self {
        Self::new()
    }
}

pub fn text_to_tokens(text: &str) -> Vec<i64> {
    KoreanG2P::new().text_to_tokens(text)
}

pub fn text_to_phonemes(text: &str) -> String {
    KoreanG2P::new().text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

/// Convert Korean word to IPA phonemes
fn word_to_phonemes(word: &str) -> String {
    let syllables: Vec<Jamo> = word.chars()
        .filter_map(decompose_hangul)
        .collect();

    if syllables.is_empty() {
        return String::new();
    }

    // Apply phonological rules
    let processed = apply_phonological_rules(&syllables);

    // Convert to IPA
    let mut phonemes = String::new();
    for jamo in processed {
        if let Some(initial) = jamo.initial {
            phonemes.push_str(choseong_to_ipa(initial));
        }
        phonemes.push_str(jungseong_to_ipa(jamo.medial));
        if let Some(final_c) = jamo.final_consonant {
            phonemes.push_str(jongseong_to_ipa(final_c));
        }
    }

    phonemes
}

/// Jamo structure representing a decomposed Hangul syllable
#[derive(Clone, Copy, Debug)]
struct Jamo {
    initial: Option<u32>,      // Choseong (initial consonant)
    medial: u32,               // Jungseong (vowel)
    final_consonant: Option<u32>, // Jongseong (final consonant)
}

/// Decompose a Hangul syllable into its jamo components
fn decompose_hangul(c: char) -> Option<Jamo> {
    let code = c as u32;

    // Check if it's a Hangul syllable (가-힣)
    if !(0xAC00..=0xD7A3).contains(&code) {
        return None;
    }

    let syllable_index = code - 0xAC00;
    let initial = syllable_index / 588;
    let medial = (syllable_index % 588) / 28;
    let final_c = syllable_index % 28;

    Some(Jamo {
        initial: Some(initial),
        medial,
        final_consonant: if final_c == 0 { None } else { Some(final_c) },
    })
}

/// Apply Korean phonological rules
fn apply_phonological_rules(syllables: &[Jamo]) -> Vec<Jamo> {
    let mut result = syllables.to_vec();

    // Process syllable pairs for sandhi rules
    for i in 0..result.len() {
        if i + 1 < result.len() {
            let current_final = result[i].final_consonant;
            let next_initial = result[i + 1].initial;

            // Liaison (연음): Final moves to next syllable if next starts with ㅇ (index 11)
            if let (Some(fc), Some(11)) = (current_final, next_initial) {
                result[i].final_consonant = None;
                result[i + 1].initial = Some(jongseong_to_choseong(fc));
            }

            // Nasalization (비음화): Stops become nasals before nasals
            if let (Some(fc), Some(ni)) = (current_final, next_initial) {
                if is_nasal_initial(ni) {
                    result[i].final_consonant = Some(nasalize_final(fc));
                }
            }
        }
    }

    result
}

fn is_nasal_initial(initial: u32) -> bool {
    // ㄴ(2), ㅁ(6)
    matches!(initial, 2 | 6)
}

fn nasalize_final(final_c: u32) -> u32 {
    match final_c {
        1 | 2 | 3 | 4 | 5 | 6 => 4,  // ㄱ,ㄲ,ㄳ,ㄴ,ㄵ,ㄶ → ㄴ (4)
        7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => 4,  // ㄷ group → ㄴ (nasalization)
        16 | 17 | 18 | 19 | 20 | 21 | 22 => 21, // ㅂ group → ㅁ (21)
        _ => final_c,
    }
}

fn jongseong_to_choseong(jongseong: u32) -> u32 {
    // Map final consonant index to initial consonant index
    match jongseong {
        1 => 0,   // ㄱ
        2 => 1,   // ㄲ
        4 => 2,   // ㄴ
        7 => 3,   // ㄷ
        8 => 5,   // ㄹ
        16 => 6,  // ㅁ
        17 => 7,  // ㅂ
        19 => 9,  // ㅅ
        21 => 11, // ㅇ
        22 => 12, // ㅈ
        23 => 14, // ㅊ
        24 => 15, // ㅋ
        25 => 16, // ㅌ
        26 => 17, // ㅍ
        27 => 18, // ㅎ
        _ => 11,  // Default to ㅇ (silent)
    }
}

/// Convert choseong (initial consonant) to IPA
fn choseong_to_ipa(index: u32) -> &'static str {
    match index {
        0 => "k",   // ㄱ
        1 => "k͈",  // ㄲ (tense)
        2 => "n",   // ㄴ
        3 => "t",   // ㄷ
        4 => "t͈",  // ㄸ (tense)
        5 => "ɾ",   // ㄹ
        6 => "m",   // ㅁ
        7 => "p",   // ㅂ
        8 => "p͈",  // ㅃ (tense)
        9 => "s",   // ㅅ
        10 => "s͈", // ㅆ (tense)
        11 => "",   // ㅇ (silent initially)
        12 => "ʧ",  // ㅈ
        13 => "ʧ͈", // ㅉ (tense)
        14 => "ʧʰ", // ㅊ (aspirated)
        15 => "kʰ", // ㅋ (aspirated)
        16 => "tʰ", // ㅌ (aspirated)
        17 => "pʰ", // ㅍ (aspirated)
        18 => "h",  // ㅎ
        _ => "",
    }
}

/// Convert jungseong (vowel) to IPA
fn jungseong_to_ipa(index: u32) -> &'static str {
    match index {
        0 => "a",    // ㅏ
        1 => "ɛ",    // ㅐ
        2 => "ja",   // ㅑ
        3 => "jɛ",   // ㅒ
        4 => "ʌ",    // ㅓ
        5 => "e",    // ㅔ
        6 => "jʌ",   // ㅕ
        7 => "je",   // ㅖ
        8 => "o",    // ㅗ
        9 => "wa",   // ㅘ
        10 => "wɛ",  // ㅙ
        11 => "we",  // ㅚ
        12 => "jo",  // ㅛ
        13 => "u",   // ㅜ
        14 => "wʌ",  // ㅝ
        15 => "we",  // ㅞ
        16 => "wi",  // ㅟ
        17 => "ju",  // ㅠ
        18 => "ɯ",   // ㅡ
        19 => "ɰi",  // ㅢ
        20 => "i",   // ㅣ
        _ => "",
    }
}

/// Convert jongseong (final consonant) to IPA
fn jongseong_to_ipa(index: u32) -> &'static str {
    match index {
        1 => "k",    // ㄱ
        2 => "k",    // ㄲ
        3 => "k",    // ㄳ
        4 => "n",    // ㄴ
        5 => "n",    // ㄵ
        6 => "n",    // ㄶ
        7 => "t",    // ㄷ
        8 => "l",    // ㄹ
        9 => "k",    // ㄺ
        10 => "m",   // ㄻ
        11 => "p",   // ㄼ
        12 => "l",   // ㄽ
        13 => "l",   // ㄾ
        14 => "l",   // ㄿ
        15 => "p",   // ㅀ
        16 => "m",   // ㅁ
        17 => "p",   // ㅂ
        18 => "p",   // ㅄ
        19 => "t",   // ㅅ
        20 => "t",   // ㅆ
        21 => "ŋ",   // ㅇ
        22 => "t",   // ㅈ
        23 => "t",   // ㅊ
        24 => "k",   // ㅋ
        25 => "t",   // ㅌ
        26 => "p",   // ㅍ
        27 => "t",   // ㅎ
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_hangul() {
        // 한 = ㅎ + ㅏ + ㄴ
        let jamo = decompose_hangul('한').unwrap();
        assert_eq!(jamo.initial, Some(18)); // ㅎ
        assert_eq!(jamo.medial, 0);         // ㅏ
        assert_eq!(jamo.final_consonant, Some(4)); // ㄴ
    }

    #[test]
    fn test_basic_conversion() {
        let g2p = KoreanG2P::new();
        let phonemes = g2p.text_to_phonemes("한국어");
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_hello() {
        let phonemes = word_to_phonemes("안녕");
        assert!(phonemes.contains('a'));
        assert!(phonemes.contains('n'));
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("안녕하세요");
        assert!(tokens.len() > 2);
    }
}
