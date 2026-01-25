//! Hanzi to Pinyin conversion
//!
//! This module handles conversion of Chinese characters (Hanzi) to Pinyin
//! with tone numbers.

use super::polyphone;
use phf::phf_map;

/// Pinyin syllable with tone
#[derive(Debug, Clone, PartialEq)]
pub struct PinyinSyllable {
    /// The pinyin syllable without tone marks (e.g., "ni")
    pub syllable: String,
    /// Tone number (1-5, where 5 is neutral/light tone)
    pub tone: u8,
}

impl PinyinSyllable {
    pub fn new(syllable: &str, tone: u8) -> Self {
        Self {
            syllable: syllable.to_string(),
            tone,
        }
    }

    /// Get pinyin with tone number suffix (e.g., "ni3")
    pub fn with_tone_number(&self) -> String {
        format!("{}{}", self.syllable, self.tone)
    }
}

/// Common single-character pinyin mappings (most frequent characters)
/// This is a subset; for full coverage, we use the polyphone dictionary
static CHAR_TO_PINYIN: phf::Map<char, &'static str> = phf_map! {
    '的' => "de5",
    '一' => "yi1",
    '是' => "shi4",
    '不' => "bu4",
    '了' => "le5",
    '在' => "zai4",
    '人' => "ren2",
    '有' => "you3",
    '我' => "wo3",
    '他' => "ta1",
    '这' => "zhe4",
    '中' => "zhong1",
    '大' => "da4",
    '来' => "lai2",
    '上' => "shang4",
    '国' => "guo2",
    '个' => "ge4",
    '到' => "dao4",
    '说' => "shuo1",
    '们' => "men5",
    '为' => "wei4",
    '子' => "zi3",
    '和' => "he2",
    '你' => "ni3",
    '地' => "di4",
    '出' => "chu1",
    '道' => "dao4",
    '也' => "ye3",
    '时' => "shi2",
    '年' => "nian2",
    '得' => "de2",
    '就' => "jiu4",
    '那' => "na4",
    '要' => "yao4",
    '下' => "xia4",
    '以' => "yi3",
    '生' => "sheng1",
    '会' => "hui4",
    '自' => "zi4",
    '着' => "zhe5",
    '去' => "qu4",
    '之' => "zhi1",
    '过' => "guo4",
    '家' => "jia1",
    '学' => "xue2",
    '对' => "dui4",
    '可' => "ke3",
    '她' => "ta1",
    '里' => "li3",
    '后' => "hou4",
    '小' => "xiao3",
    '么' => "me5",
    '心' => "xin1",
    '多' => "duo1",
    '天' => "tian1",
    '而' => "er2",
    '能' => "neng2",
    '好' => "hao3",
    '都' => "dou1",
    '然' => "ran2",
    '没' => "mei2",
    '日' => "ri4",
    '于' => "yu2",
    '起' => "qi3",
    '还' => "hai2",
    '发' => "fa1",
    '成' => "cheng2",
    '事' => "shi4",
    '只' => "zhi3",
    '作' => "zuo4",
    '当' => "dang1",
    '想' => "xiang3",
    '看' => "kan4",
    '文' => "wen2",
    '无' => "wu2",
    '开' => "kai1",
    '手' => "shou3",
    '十' => "shi2",
    '用' => "yong4",
    '主' => "zhu3",
    '行' => "xing2",
    '方' => "fang1",
    '又' => "you4",
    '如' => "ru2",
    '前' => "qian2",
    '所' => "suo3",
    '本' => "ben3",
    '见' => "jian4",
    '经' => "jing1",
    '头' => "tou2",
    '面' => "mian4",
    '把' => "ba3",
    '问' => "wen4",
    '样' => "yang4",
    '定' => "ding4",
    '长' => "chang2",
    '很' => "hen3",
    '女' => "nv3",
    '些' => "xie1",
    '名' => "ming2",
    '外' => "wai4",
    '却' => "que4",
    '让' => "rang4",
    '被' => "bei4",
    '点' => "dian3",
    '高' => "gao1",
    '走' => "zou3",
    '世' => "shi4",
    '界' => "jie4",
    '万' => "wan4",
    '百' => "bai3",
    '千' => "qian1",
    '零' => "ling2",
    '二' => "er4",
    '三' => "san1",
    '四' => "si4",
    '五' => "wu3",
    '六' => "liu4",
    '七' => "qi1",
    '八' => "ba1",
    '九' => "jiu3",
    '元' => "yuan2",
    '角' => "jiao3",
    '分' => "fen1",
    '新' => "xin1",
    '加' => "jia1",
    '坡' => "po1",
    '美' => "mei3",
    '月' => "yue4",
};

/// Parse a pinyin string with tone number (e.g., "ni3") into PinyinSyllable
pub fn parse_pinyin(pinyin: &str) -> PinyinSyllable {
    if pinyin.is_empty() {
        return PinyinSyllable::new("", 5);
    }

    let last_char = pinyin.chars().last().unwrap();
    if last_char.is_ascii_digit() {
        let tone = last_char.to_digit(10).unwrap_or(5) as u8;
        let syllable = &pinyin[..pinyin.len() - 1];
        PinyinSyllable::new(syllable, tone)
    } else {
        PinyinSyllable::new(pinyin, 5)
    }
}

/// Convert a single character to pinyin
pub fn char_to_pinyin(c: char) -> Option<PinyinSyllable> {
    // First check our static map
    if let Some(&pinyin) = CHAR_TO_PINYIN.get(&c) {
        return Some(parse_pinyin(pinyin));
    }

    // Check polyphone dictionary for default reading
    if let Some(pinyin) = polyphone::get_default_pinyin(c) {
        return Some(parse_pinyin(pinyin));
    }

    None
}

/// Convert a word to pinyin with POS tag for disambiguation
pub fn to_pinyin_with_pos(word: &str, pos: &str) -> Vec<PinyinSyllable> {
    // First check if this exact phrase has a polyphone entry
    if let Some(pinyins) = polyphone::lookup_phrase(word) {
        return pinyins
            .split(' ')
            .map(|p| parse_pinyin(p))
            .collect();
    }

    // Otherwise, convert character by character
    let mut result = Vec::new();
    for c in word.chars() {
        // Skip non-Chinese characters
        if !super::segmenter::Segmenter::is_chinese_char(c) {
            continue;
        }

        // Try POS-based disambiguation first
        if let Some(pinyin) = polyphone::lookup_with_pos(c, pos) {
            result.push(parse_pinyin(pinyin));
        } else if let Some(pinyin) = char_to_pinyin(c) {
            result.push(pinyin);
        }
    }

    result
}

/// Convert a string of Chinese characters to pinyin (basic conversion)
pub fn to_pinyin(text: &str) -> Vec<PinyinSyllable> {
    to_pinyin_with_pos(text, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pinyin() {
        let p = parse_pinyin("ni3");
        assert_eq!(p.syllable, "ni");
        assert_eq!(p.tone, 3);
    }

    #[test]
    fn test_parse_pinyin_neutral() {
        let p = parse_pinyin("de5");
        assert_eq!(p.syllable, "de");
        assert_eq!(p.tone, 5);
    }

    #[test]
    fn test_char_to_pinyin() {
        let p = char_to_pinyin('你').unwrap();
        assert_eq!(p.syllable, "ni");
        assert_eq!(p.tone, 3);
    }

    #[test]
    fn test_to_pinyin() {
        let result = to_pinyin("你好");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].syllable, "ni");
        assert_eq!(result[0].tone, 3);
        assert_eq!(result[1].syllable, "hao");
        assert_eq!(result[1].tone, 3);
    }

    #[test]
    fn test_number_chars() {
        let result = to_pinyin("一二三");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].syllable, "yi");
        assert_eq!(result[0].tone, 1);
    }
}
