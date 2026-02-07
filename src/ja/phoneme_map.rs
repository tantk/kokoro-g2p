//! Japanese katakana to IPA phoneme mapping.
//!
//! Based on Misaki G2P: https://github.com/hexgrad/misaki
//! Maps katakana characters to IPA phonemes used by Kokoro TTS.

use lazy_static::lazy_static;
use phf::phf_map;
use std::collections::HashMap;

/// Single katakana to phoneme mapping
pub static SINGLE_KANA: phf::Map<char, &'static str> = phf_map! {
    // Small vowels
    'ァ' => "a",
    'ア' => "a",
    'ィ' => "i",
    'イ' => "i",
    'ゥ' => "u",
    'ウ' => "u",
    'ェ' => "e",
    'エ' => "e",
    'ォ' => "o",
    'オ' => "o",

    // K row
    'カ' => "ka",
    'ガ' => "ga",
    'キ' => "ki",
    'ギ' => "gi",
    'ク' => "ku",
    'グ' => "gu",
    'ケ' => "ke",
    'ゲ' => "ge",
    'コ' => "ko",
    'ゴ' => "go",

    // S row
    'サ' => "sa",
    'ザ' => "za",
    'シ' => "ɕi",
    'ジ' => "ʥi",
    'ス' => "su",
    'ズ' => "zu",
    'セ' => "se",
    'ゼ' => "ze",
    'ソ' => "so",
    'ゾ' => "zo",

    // T row
    'タ' => "ta",
    'ダ' => "da",
    'チ' => "ʨi",
    'ヂ' => "ʥi",
    'ツ' => "ʦu",
    'ヅ' => "zu",
    'テ' => "te",
    'デ' => "de",
    'ト' => "to",
    'ド' => "do",

    // N row
    'ナ' => "na",
    'ニ' => "ni",
    'ヌ' => "nu",
    'ネ' => "ne",
    'ノ' => "no",

    // H row
    'ハ' => "ha",
    'バ' => "ba",
    'パ' => "pa",
    'ヒ' => "hi",
    'ビ' => "bi",
    'ピ' => "pi",
    'フ' => "fu",
    'ブ' => "bu",
    'プ' => "pu",
    'ヘ' => "he",
    'ベ' => "be",
    'ペ' => "pe",
    'ホ' => "ho",
    'ボ' => "bo",
    'ポ' => "po",

    // M row
    'マ' => "ma",
    'ミ' => "mi",
    'ム' => "mu",
    'メ' => "me",
    'モ' => "mo",

    // Y row
    'ャ' => "ja",
    'ヤ' => "ja",
    'ュ' => "ju",
    'ユ' => "ju",
    'ョ' => "jo",
    'ヨ' => "jo",

    // R row
    'ラ' => "ra",
    'リ' => "ri",
    'ル' => "ru",
    'レ' => "re",
    'ロ' => "ro",

    // W row
    'ヮ' => "wa",
    'ワ' => "wa",
    'ヰ' => "i",
    'ヱ' => "e",
    'ヲ' => "o",

    // Special
    'ッ' => "ʔ",  // Glottal stop (geminate)
    'ン' => "ɴ",  // Moraic nasal
    'ー' => "ː",  // Long vowel

    // Extended katakana
    'ヴ' => "vu",
    'ヵ' => "ka",
    'ヶ' => "ke",
    'ヷ' => "va",
    'ヸ' => "vi",
    'ヹ' => "ve",
    'ヺ' => "vo",
};

lazy_static! {
    /// Combined katakana sequences to phoneme mapping
    pub static ref COMBINED_KANA: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();

        // イ combinations
        m.insert("イェ", "je");

        // ウ combinations
        m.insert("ウィ", "wi");
        m.insert("ウゥ", "wu");
        m.insert("ウェ", "we");
        m.insert("ウォ", "wo");

        // キ combinations (palatalized)
        m.insert("キィ", "ᶄi");
        m.insert("キェ", "ᶄe");
        m.insert("キャ", "ᶄa");
        m.insert("キュ", "ᶄu");
        m.insert("キョ", "ᶄo");

        // ギ combinations
        m.insert("ギィ", "ᶃi");
        m.insert("ギェ", "ᶃe");
        m.insert("ギャ", "ᶃa");
        m.insert("ギュ", "ᶃu");
        m.insert("ギョ", "ᶃo");

        // ク combinations (labialized)
        m.insert("クァ", "Ka");
        m.insert("クィ", "Ki");
        m.insert("クゥ", "Ku");
        m.insert("クェ", "Ke");
        m.insert("クォ", "Ko");
        m.insert("クヮ", "Ka");

        // グ combinations
        m.insert("グァ", "Ga");
        m.insert("グィ", "Gi");
        m.insert("グゥ", "Gu");
        m.insert("グェ", "Ge");
        m.insert("グォ", "Go");
        m.insert("グヮ", "Ga");

        // シ combinations
        m.insert("シェ", "ɕe");
        m.insert("シャ", "ɕa");
        m.insert("シュ", "ɕu");
        m.insert("ショ", "ɕo");

        // ジ combinations
        m.insert("ジェ", "ʥe");
        m.insert("ジャ", "ʥa");
        m.insert("ジュ", "ʥu");
        m.insert("ジョ", "ʥo");

        // ス combinations
        m.insert("スィ", "si");

        // ズ combinations
        m.insert("ズィ", "zi");

        // チ combinations
        m.insert("チェ", "ʨe");
        m.insert("チャ", "ʨa");
        m.insert("チュ", "ʨu");
        m.insert("チョ", "ʨo");

        // ヂ combinations
        m.insert("ヂェ", "ʥe");
        m.insert("ヂャ", "ʥa");
        m.insert("ヂュ", "ʥu");
        m.insert("ヂョ", "ʥo");

        // ツ combinations
        m.insert("ツァ", "ʦa");
        m.insert("ツィ", "ʦi");
        m.insert("ツェ", "ʦe");
        m.insert("ツォ", "ʦo");

        // テ combinations
        m.insert("ティ", "ti");
        m.insert("テェ", "ƫe");
        m.insert("テャ", "ƫa");
        m.insert("テュ", "ƫu");
        m.insert("テョ", "ƫo");

        // デ combinations
        m.insert("ディ", "di");
        m.insert("デェ", "ᶁe");
        m.insert("デャ", "ᶁa");
        m.insert("デュ", "ᶁu");
        m.insert("デョ", "ᶁo");

        // ト/ド combinations
        m.insert("トゥ", "tu");
        m.insert("ドゥ", "du");

        // ニ combinations
        m.insert("ニィ", "ɲi");
        m.insert("ニェ", "ɲe");
        m.insert("ニャ", "ɲa");
        m.insert("ニュ", "ɲu");
        m.insert("ニョ", "ɲo");

        // ヒ combinations
        m.insert("ヒィ", "çi");
        m.insert("ヒェ", "çe");
        m.insert("ヒャ", "ça");
        m.insert("ヒュ", "çu");
        m.insert("ヒョ", "ço");

        // ビ combinations
        m.insert("ビィ", "ᶀi");
        m.insert("ビェ", "ᶀe");
        m.insert("ビャ", "ᶀa");
        m.insert("ビュ", "ᶀu");
        m.insert("ビョ", "ᶀo");

        // ピ combinations
        m.insert("ピィ", "ᶈi");
        m.insert("ピェ", "ᶈe");
        m.insert("ピャ", "ᶈa");
        m.insert("ピュ", "ᶈu");
        m.insert("ピョ", "ᶈo");

        // フ combinations
        m.insert("ファ", "fa");
        m.insert("フィ", "fi");
        m.insert("フェ", "fe");
        m.insert("フォ", "fo");

        // ミ combinations
        m.insert("ミィ", "ᶆi");
        m.insert("ミェ", "ᶆe");
        m.insert("ミャ", "ᶆa");
        m.insert("ミュ", "ᶆu");
        m.insert("ミョ", "ᶆo");

        // リ combinations
        m.insert("リィ", "ᶉi");
        m.insert("リェ", "ᶉe");
        m.insert("リャ", "ᶉa");
        m.insert("リュ", "ᶉu");
        m.insert("リョ", "ᶉo");

        // ヴ combinations
        m.insert("ヴァ", "va");
        m.insert("ヴィ", "vi");
        m.insert("ヴェ", "ve");
        m.insert("ヴォ", "vo");
        m.insert("ヴャ", "ᶀa");
        m.insert("ヴュ", "ᶀu");
        m.insert("ヴョ", "ᶀo");

        m
    };

    /// Japanese punctuation to ASCII mapping
    pub static ref PUNCT_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('«', '"');
        m.insert('»', '"');
        m.insert('、', ',');
        m.insert('。', '.');
        m.insert('〈', '"');
        m.insert('〉', '"');
        m.insert('《', '"');
        m.insert('》', '"');
        m.insert('「', '"');
        m.insert('」', '"');
        m.insert('『', '"');
        m.insert('』', '"');
        m.insert('【', '"');
        m.insert('】', '"');
        m.insert('！', '!');
        m.insert('（', '(');
        m.insert('）', ')');
        m.insert('：', ':');
        m.insert('；', ';');
        m.insert('？', '?');
        m.insert('　', ' '); // Full-width space
        m
    };
}

/// Check if a character is a katakana character
pub fn is_katakana(c: char) -> bool {
    ('\u{30A0}'..='\u{30FF}').contains(&c) || c == 'ー'
}

/// Check if a character is a hiragana character
pub fn is_hiragana(c: char) -> bool {
    ('\u{3040}'..='\u{309F}').contains(&c)
}

/// Convert hiragana to katakana
pub fn hiragana_to_katakana(c: char) -> char {
    if is_hiragana(c) {
        // Hiragana range is U+3040-U+309F, Katakana is U+30A0-U+30FF
        // Offset is 0x60
        char::from_u32(c as u32 + 0x60).unwrap_or(c)
    } else {
        c
    }
}

/// Convert a string from hiragana to katakana
pub fn str_hiragana_to_katakana(s: &str) -> String {
    s.chars().map(hiragana_to_katakana).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_kana() {
        assert_eq!(SINGLE_KANA.get(&'ア'), Some(&"a"));
        assert_eq!(SINGLE_KANA.get(&'カ'), Some(&"ka"));
        assert_eq!(SINGLE_KANA.get(&'シ'), Some(&"ɕi"));
        assert_eq!(SINGLE_KANA.get(&'ッ'), Some(&"ʔ"));
        assert_eq!(SINGLE_KANA.get(&'ン'), Some(&"ɴ"));
    }

    #[test]
    fn test_combined_kana() {
        assert_eq!(COMBINED_KANA.get("シャ"), Some(&"ɕa"));
        assert_eq!(COMBINED_KANA.get("チュ"), Some(&"ʨu"));
        assert_eq!(COMBINED_KANA.get("ニャ"), Some(&"ɲa"));
    }

    #[test]
    fn test_hiragana_to_katakana() {
        assert_eq!(hiragana_to_katakana('あ'), 'ア');
        assert_eq!(hiragana_to_katakana('か'), 'カ');
        assert_eq!(hiragana_to_katakana('ん'), 'ン');
        assert_eq!(str_hiragana_to_katakana("こんにちは"), "コンニチハ");
    }
}
