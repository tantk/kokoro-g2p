//! Vietnamese G2P (Grapheme-to-Phoneme) module
//!
//! Vietnamese (Quốc Ngữ) uses Latin script with extensive diacritics:
//! - 6 tones (Northern dialect): ngang, huyền, sắc, hỏi, ngã, nặng
//! - Vowel modifications: ă, â, ê, ô, ơ, ư
//! - All tone marks are written, making G2P relatively straightforward
//!
//! This implementation defaults to Northern (Hanoi) Vietnamese pronunciation.

pub mod normalizer;

use crate::tokenizer;

/// Vietnamese G2P processor
pub struct VietnameseG2P;

impl VietnameseG2P {
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

impl Default for VietnameseG2P {
    fn default() -> Self {
        Self::new()
    }
}

pub fn text_to_tokens(text: &str) -> Vec<i64> {
    VietnameseG2P::new().text_to_tokens(text)
}

pub fn text_to_phonemes(text: &str) -> String {
    VietnameseG2P::new().text_to_phonemes(text)
}

fn is_punctuation(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '.' | ',' | '!' | '?' | ';' | ':' | '—' | '…' | '"' | '(' | ')'))
}

/// Convert Vietnamese word to IPA phonemes with tone markers
fn word_to_phonemes(word: &str) -> String {
    let word_lower = word.to_lowercase();
    let chars: Vec<char> = word_lower.chars().collect();
    let mut phonemes = String::new();
    let mut i = 0;

    // Detect tone from vowel diacritics
    let tone = detect_tone(&chars);

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let next2 = chars.get(i + 2).copied();

        // Trigraphs
        match (c, next, next2) {
            // ngh → /ŋ/ before i, e
            ('n', Some('g'), Some('h')) => {
                phonemes.push('ŋ');
                i += 3;
                continue;
            }
            _ => {}
        }

        // Digraphs
        match (c, next) {
            // ng → /ŋ/
            ('n', Some('g')) => {
                phonemes.push('ŋ');
                i += 2;
                continue;
            }
            // nh → /ɲ/
            ('n', Some('h')) => {
                phonemes.push('ɲ');
                i += 2;
                continue;
            }
            // ch → /c/ or /ʧ/
            ('c', Some('h')) => {
                phonemes.push('c');
                i += 2;
                continue;
            }
            // th → /tʰ/
            ('t', Some('h')) => {
                phonemes.push('t');
                phonemes.push('ʰ');
                i += 2;
                continue;
            }
            // tr → /ʈ/ (retroflex)
            ('t', Some('r')) => {
                phonemes.push('ʈ');
                i += 2;
                continue;
            }
            // ph → /f/
            ('p', Some('h')) => {
                phonemes.push('f');
                i += 2;
                continue;
            }
            // kh → /x/
            ('k', Some('h')) => {
                phonemes.push('x');
                i += 2;
                continue;
            }
            // gh → /ɣ/ (before i, e)
            ('g', Some('h')) => {
                phonemes.push('ɣ');
                i += 2;
                continue;
            }
            // gi → /z/
            ('g', Some('i')) => {
                phonemes.push('z');
                i += 2;
                continue;
            }
            // qu → /kw/
            ('q', Some('u')) => {
                phonemes.push('k');
                phonemes.push('w');
                i += 2;
                continue;
            }
            // Diphthongs
            ('a', Some('i')) => {
                phonemes.push('a');
                phonemes.push('j');
                i += 2;
                continue;
            }
            ('a', Some('o')) | ('a', Some('u')) => {
                phonemes.push('a');
                phonemes.push('w');
                i += 2;
                continue;
            }
            ('o', Some('i')) | ('ô', Some('i')) => {
                phonemes.push('o');
                phonemes.push('j');
                i += 2;
                continue;
            }
            ('u', Some('i')) | ('ư', Some('i')) => {
                phonemes.push('u');
                phonemes.push('j');
                i += 2;
                continue;
            }
            _ => {}
        }

        // Single character conversions
        match c {
            // Vowels with tones (base vowel without tone marking in phonemes)
            'a' | 'à' | 'á' | 'ả' | 'ã' | 'ạ' => phonemes.push('a'),
            'ă' | 'ằ' | 'ắ' | 'ẳ' | 'ẵ' | 'ặ' => phonemes.push('ă'),
            'â' | 'ầ' | 'ấ' | 'ẩ' | 'ẫ' | 'ậ' => phonemes.push('ə'),
            'e' | 'è' | 'é' | 'ẻ' | 'ẽ' | 'ẹ' => phonemes.push('ɛ'),
            'ê' | 'ề' | 'ế' | 'ể' | 'ễ' | 'ệ' => phonemes.push('e'),
            'i' | 'ì' | 'í' | 'ỉ' | 'ĩ' | 'ị' => phonemes.push('i'),
            'y' | 'ỳ' | 'ý' | 'ỷ' | 'ỹ' | 'ỵ' => phonemes.push('i'),
            'o' | 'ò' | 'ó' | 'ỏ' | 'õ' | 'ọ' => phonemes.push('ɔ'),
            'ô' | 'ồ' | 'ố' | 'ổ' | 'ỗ' | 'ộ' => phonemes.push('o'),
            'ơ' | 'ờ' | 'ớ' | 'ở' | 'ỡ' | 'ợ' => phonemes.push('ɤ'),
            'u' | 'ù' | 'ú' | 'ủ' | 'ũ' | 'ụ' => phonemes.push('u'),
            'ư' | 'ừ' | 'ứ' | 'ử' | 'ữ' | 'ự' => phonemes.push('ɯ'),

            // Consonants
            'b' => phonemes.push('ɓ'), // Implosive
            'c' => phonemes.push('k'),
            'd' => phonemes.push('z'),  // Northern Vietnamese
            'đ' => phonemes.push('d'),
            'g' => phonemes.push('ɣ'),
            'h' => phonemes.push('h'),
            'k' => phonemes.push('k'),
            'l' => phonemes.push('l'),
            'm' => phonemes.push('m'),
            'n' => phonemes.push('n'),
            'p' => phonemes.push('p'),
            'r' => phonemes.push('z'),  // Northern Vietnamese (varies by dialect)
            's' => phonemes.push('s'),
            't' => phonemes.push('t'),
            'v' => phonemes.push('v'),
            'x' => phonemes.push('s'),

            '.' | ',' | '!' | '?' | ';' | ':' => phonemes.push(c),
            _ => {}
        }

        i += 1;
    }

    // Add tone marker at end
    phonemes.push_str(tone_to_marker(tone));

    phonemes
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tone {
    Ngang,  // Level (no mark)
    Huyen,  // Falling (grave)
    Sac,    // Rising (acute)
    Hoi,    // Dipping-rising (hook)
    Nga,    // Rising glottalized (tilde)
    Nang,   // Low falling (dot below)
}

fn detect_tone(chars: &[char]) -> Tone {
    for &c in chars {
        match c {
            // Huyền (grave accent) - falling
            'à' | 'ằ' | 'ầ' | 'è' | 'ề' | 'ì' | 'ò' | 'ồ' | 'ờ' | 'ù' | 'ừ' | 'ỳ' => return Tone::Huyen,
            // Sắc (acute accent) - rising
            'á' | 'ắ' | 'ấ' | 'é' | 'ế' | 'í' | 'ó' | 'ố' | 'ớ' | 'ú' | 'ứ' | 'ý' => return Tone::Sac,
            // Hỏi (hook above) - dipping
            'ả' | 'ẳ' | 'ẩ' | 'ẻ' | 'ể' | 'ỉ' | 'ỏ' | 'ổ' | 'ở' | 'ủ' | 'ử' | 'ỷ' => return Tone::Hoi,
            // Ngã (tilde) - rising glottalized
            'ã' | 'ẵ' | 'ẫ' | 'ẽ' | 'ễ' | 'ĩ' | 'õ' | 'ỗ' | 'ỡ' | 'ũ' | 'ữ' | 'ỹ' => return Tone::Nga,
            // Nặng (dot below) - low falling
            'ạ' | 'ặ' | 'ậ' | 'ẹ' | 'ệ' | 'ị' | 'ọ' | 'ộ' | 'ợ' | 'ụ' | 'ự' | 'ỵ' => return Tone::Nang,
            _ => {}
        }
    }
    Tone::Ngang // Default: no tone mark = level tone
}

fn tone_to_marker(tone: Tone) -> &'static str {
    match tone {
        Tone::Ngang => "→",  // Level
        Tone::Huyen => "↘",  // Falling
        Tone::Sac => "↗",    // Rising
        Tone::Hoi => "↓",    // Dipping (down then up, simplified)
        Tone::Nga => "↗",    // Rising with glottal (simplified to rising)
        Tone::Nang => "↘",   // Low falling
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = VietnameseG2P::new();
        let phonemes = g2p.text_to_phonemes("xin chào");
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_tone_detection() {
        let chars: Vec<char> = "chào".chars().collect();
        assert_eq!(detect_tone(&chars), Tone::Huyen);

        let chars: Vec<char> = "có".chars().collect();
        assert_eq!(detect_tone(&chars), Tone::Sac);
    }

    #[test]
    fn test_digraphs() {
        let phonemes = word_to_phonemes("nghe");
        assert!(phonemes.contains('ŋ'));

        let phonemes = word_to_phonemes("nhà");
        assert!(phonemes.contains('ɲ'));
    }

    #[test]
    fn test_d_and_đ() {
        // d → /z/ in Northern Vietnamese
        let phonemes = word_to_phonemes("da");
        assert!(phonemes.contains('z'));

        // đ → /d/
        let phonemes = word_to_phonemes("đi");
        assert!(phonemes.contains('d'));
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("xin chào");
        assert!(tokens.len() > 2);
    }
}
