//! Pinyin to Zhuyin (Bopomofo) phoneme mapping
//!
//! This module converts Pinyin syllables with tone numbers to Zhuyin-based
//! phonemes suitable for the kokoro-v1.1-zh ONNX model.
//!
//! Zhuyin (注音符號) is a phonetic system for Mandarin Chinese.
//! The Kokoro Chinese model uses Zhuyin symbols with tone markers.

use super::pinyin::PinyinSyllable;
use phf::phf_map;

/// Tone markers for Kokoro Chinese model
/// These map to the intonation markers already in the vocabulary:
/// - Tone 1 (level): → (171)
/// - Tone 2 (rising): ↗ (172)
/// - Tone 3 (dipping): ↓ (169)
/// - Tone 4 (falling): ↘ (173)
/// - Tone 5 (neutral): no marker
const TONE_MARKERS: [char; 6] = [
    ' ',  // 0: unused
    '→',  // 1: level tone (yīn píng)
    '↗',  // 2: rising tone (yáng píng)
    '↓',  // 3: dipping tone (shǎng shēng)
    '↘',  // 4: falling tone (qù shēng)
    ' ',  // 5: neutral tone (no marker)
];

/// Pinyin initials to Zhuyin mapping
static INITIALS: phf::Map<&'static str, &'static str> = phf_map! {
    "b" => "ㄅ",
    "p" => "ㄆ",
    "m" => "ㄇ",
    "f" => "ㄈ",
    "d" => "ㄉ",
    "t" => "ㄊ",
    "n" => "ㄋ",
    "l" => "ㄌ",
    "g" => "ㄍ",
    "k" => "ㄎ",
    "h" => "ㄏ",
    "j" => "ㄐ",
    "q" => "ㄑ",
    "x" => "ㄒ",
    "zh" => "ㄓ",
    "ch" => "ㄔ",
    "sh" => "ㄕ",
    "r" => "ㄖ",
    "z" => "ㄗ",
    "c" => "ㄘ",
    "s" => "ㄙ",
    "y" => "ㄧ",  // Can also be part of medial
    "w" => "ㄨ",  // Can also be part of medial
};

/// Pinyin finals to Zhuyin mapping
/// This handles the complete final (medial + nucleus + coda)
static FINALS: phf::Map<&'static str, &'static str> = phf_map! {
    // Basic vowels
    "a" => "ㄚ",
    "o" => "ㄛ",
    "e" => "ㄜ",
    "i" => "ㄧ",
    "u" => "ㄨ",
    "v" => "ㄩ",  // ü represented as v
    "ü" => "ㄩ",

    // A combinations
    "ai" => "ㄞ",
    "ao" => "ㄠ",
    "an" => "ㄢ",
    "ang" => "ㄤ",

    // E combinations
    "ei" => "ㄟ",
    "en" => "ㄣ",
    "eng" => "ㄥ",
    "er" => "ㄦ",

    // O combinations
    "ou" => "ㄡ",
    "ong" => "ㄨㄥ",

    // I combinations
    "ia" => "ㄧㄚ",
    "ie" => "ㄧㄝ",
    "iao" => "ㄧㄠ",
    "iu" => "ㄧㄡ",
    "ian" => "ㄧㄢ",
    "in" => "ㄧㄣ",
    "iang" => "ㄧㄤ",
    "ing" => "ㄧㄥ",
    "iong" => "ㄩㄥ",

    // U combinations
    "ua" => "ㄨㄚ",
    "uo" => "ㄨㄛ",
    "uai" => "ㄨㄞ",
    "ui" => "ㄨㄟ",
    "uan" => "ㄨㄢ",
    "un" => "ㄨㄣ",
    "uang" => "ㄨㄤ",
    "ueng" => "ㄨㄥ",

    // Ü combinations
    "ve" => "ㄩㄝ",
    "üe" => "ㄩㄝ",
    "van" => "ㄩㄢ",
    "üan" => "ㄩㄢ",
    "vn" => "ㄩㄣ",
    "ün" => "ㄩㄣ",

    // Note: "i" already defined in basic vowels above
};

/// Complete syllable to Zhuyin mapping for special cases
static SYLLABLES: phf::Map<&'static str, &'static str> = phf_map! {
    // Syllables with special handling
    "zhi" => "ㄓ",
    "chi" => "ㄔ",
    "shi" => "ㄕ",
    "ri" => "ㄖ",
    "zi" => "ㄗ",
    "ci" => "ㄘ",
    "si" => "ㄙ",

    // yi, wu, yu series (standalone)
    "yi" => "ㄧ",
    "ya" => "ㄧㄚ",
    "ye" => "ㄧㄝ",
    "yao" => "ㄧㄠ",
    "you" => "ㄧㄡ",
    "yan" => "ㄧㄢ",
    "yin" => "ㄧㄣ",
    "yang" => "ㄧㄤ",
    "ying" => "ㄧㄥ",
    "yong" => "ㄩㄥ",

    "wu" => "ㄨ",
    "wa" => "ㄨㄚ",
    "wo" => "ㄨㄛ",
    "wai" => "ㄨㄞ",
    "wei" => "ㄨㄟ",
    "wan" => "ㄨㄢ",
    "wen" => "ㄨㄣ",
    "wang" => "ㄨㄤ",
    "weng" => "ㄨㄥ",

    "yu" => "ㄩ",
    "yue" => "ㄩㄝ",
    "yuan" => "ㄩㄢ",
    "yun" => "ㄩㄣ",

    // nü, lü special cases
    "nv" => "ㄋㄩ",
    "nü" => "ㄋㄩ",
    "lv" => "ㄌㄩ",
    "lü" => "ㄌㄩ",
    "nve" => "ㄋㄩㄝ",
    "nüe" => "ㄋㄩㄝ",
    "lve" => "ㄌㄩㄝ",
    "lüe" => "ㄌㄩㄝ",

    // Common full syllables
    "a" => "ㄚ",
    "o" => "ㄛ",
    "e" => "ㄜ",
    "ai" => "ㄞ",
    "ei" => "ㄟ",
    "ao" => "ㄠ",
    "ou" => "ㄡ",
    "an" => "ㄢ",
    "en" => "ㄣ",
    "ang" => "ㄤ",
    "eng" => "ㄥ",
    "er" => "ㄦ",

    // j, q, x + ü
    "ju" => "ㄐㄩ",
    "qu" => "ㄑㄩ",
    "xu" => "ㄒㄩ",
    "jue" => "ㄐㄩㄝ",
    "que" => "ㄑㄩㄝ",
    "xue" => "ㄒㄩㄝ",
    "juan" => "ㄐㄩㄢ",
    "quan" => "ㄑㄩㄢ",
    "xuan" => "ㄒㄩㄢ",
    "jun" => "ㄐㄩㄣ",
    "qun" => "ㄑㄩㄣ",
    "xun" => "ㄒㄩㄣ",
};

/// Convert a pinyin syllable to Zhuyin
fn syllable_to_zhuyin(syllable: &str) -> String {
    let syllable_lower = syllable.to_lowercase();

    // First check complete syllable mapping
    if let Some(&zhuyin) = SYLLABLES.get(syllable_lower.as_str()) {
        return zhuyin.to_string();
    }

    // Otherwise, decompose into initial + final
    let (initial, final_part) = split_initial_final(&syllable_lower);

    let mut result = String::new();

    // Add initial
    if !initial.is_empty() {
        if let Some(&zhuyin_initial) = INITIALS.get(initial) {
            result.push_str(zhuyin_initial);
        }
    }

    // Add final
    if !final_part.is_empty() {
        if let Some(&zhuyin_final) = FINALS.get(final_part) {
            result.push_str(zhuyin_final);
        } else {
            // Try to decompose further or use character-by-character
            for c in final_part.chars() {
                if let Some(&zhuyin_char) = FINALS.get(&c.to_string() as &str) {
                    result.push_str(zhuyin_char);
                }
            }
        }
    }

    // If still empty, return the original syllable
    if result.is_empty() {
        syllable.to_string()
    } else {
        result
    }
}

/// Split a pinyin syllable into initial and final
fn split_initial_final(syllable: &str) -> (&str, &str) {
    // Check for two-character initials first
    if syllable.len() >= 2 {
        let two_chars = &syllable[..2];
        if matches!(two_chars, "zh" | "ch" | "sh") {
            return (two_chars, &syllable[2..]);
        }
    }

    // Check for single-character initials
    if !syllable.is_empty() {
        let first = &syllable[..1];
        if matches!(
            first,
            "b" | "p" | "m" | "f" | "d" | "t" | "n" | "l" | "g" | "k" | "h" | "j" | "q" | "x"
                | "r" | "z" | "c" | "s"
        ) {
            return (first, &syllable[1..]);
        }
    }

    // No initial (starts with vowel or y/w)
    ("", syllable)
}

/// Convert a sequence of PinyinSyllables to Zhuyin string with tone markers
pub fn pinyin_to_zhuyin(syllables: &[PinyinSyllable]) -> String {
    let mut result = String::new();

    for (i, syl) in syllables.iter().enumerate() {
        // Add space between syllables (but not at the start)
        if i > 0 {
            result.push(' ');
        }

        // Convert syllable to Zhuyin
        let zhuyin = syllable_to_zhuyin(&syl.syllable);
        result.push_str(&zhuyin);

        // Add tone marker (except for neutral tone 5)
        if syl.tone >= 1 && syl.tone <= 4 {
            result.push(TONE_MARKERS[syl.tone as usize]);
        }
    }

    result
}

/// Convert a single pinyin string with tone number to Zhuyin
pub fn convert_single(pinyin_with_tone: &str) -> String {
    use super::pinyin::parse_pinyin;
    let syl = parse_pinyin(pinyin_with_tone);
    let syllables = vec![syl];
    pinyin_to_zhuyin(&syllables)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syllable_to_zhuyin() {
        assert_eq!(syllable_to_zhuyin("ni"), "ㄋㄧ");
        assert_eq!(syllable_to_zhuyin("hao"), "ㄏㄠ");
        assert_eq!(syllable_to_zhuyin("shi"), "ㄕ");
        assert_eq!(syllable_to_zhuyin("zhi"), "ㄓ");
    }

    #[test]
    fn test_yi_wu_yu() {
        assert_eq!(syllable_to_zhuyin("yi"), "ㄧ");
        assert_eq!(syllable_to_zhuyin("wu"), "ㄨ");
        assert_eq!(syllable_to_zhuyin("yu"), "ㄩ");
    }

    #[test]
    fn test_j_q_x_u() {
        assert_eq!(syllable_to_zhuyin("ju"), "ㄐㄩ");
        assert_eq!(syllable_to_zhuyin("qu"), "ㄑㄩ");
        assert_eq!(syllable_to_zhuyin("xu"), "ㄒㄩ");
    }

    #[test]
    fn test_pinyin_to_zhuyin_with_tones() {
        let syllables = vec![
            PinyinSyllable::new("ni", 2), // After sandhi
            PinyinSyllable::new("hao", 3),
        ];
        let result = pinyin_to_zhuyin(&syllables);
        // Should have Zhuyin + tone markers
        assert!(result.contains("ㄋ"));
        assert!(result.contains("ㄏ"));
    }

    #[test]
    fn test_tone_markers() {
        let syllables = vec![
            PinyinSyllable::new("ma", 1),
            PinyinSyllable::new("ma", 2),
            PinyinSyllable::new("ma", 3),
            PinyinSyllable::new("ma", 4),
        ];
        let result = pinyin_to_zhuyin(&syllables);
        assert!(result.contains('→')); // Tone 1
        assert!(result.contains('↗')); // Tone 2
        assert!(result.contains('↓')); // Tone 3
        assert!(result.contains('↘')); // Tone 4
    }

    #[test]
    fn test_convert_single() {
        let result = convert_single("ni3");
        assert!(result.contains("ㄋ"));
        assert!(result.contains('↓')); // Tone 3
    }
}
