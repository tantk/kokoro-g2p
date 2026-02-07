//! Japanese kanji to katakana reading conversion.
//!
//! Uses a compressed dictionary for common words and their readings.

use lazy_static::lazy_static;
use phf::phf_map;
use std::collections::HashMap;

/// Common Japanese words with their katakana readings.
/// Format: kanji/mixed -> katakana reading
///
/// This is a subset of the most common words. For better coverage,
/// this could be expanded with a larger dictionary.
pub static READINGS: phf::Map<&'static str, &'static str> = phf_map! {
    // Greetings
    "今日は" => "コンニチハ",
    "こんにちは" => "コンニチハ",
    "今晩は" => "コンバンハ",
    "こんばんは" => "コンバンハ",
    "お早う" => "オハヨウ",
    "おはよう" => "オハヨウ",
    "有難う" => "アリガトウ",
    "ありがとう" => "アリガトウ",
    "御免" => "ゴメン",
    "すみません" => "スミマセン",
    "さようなら" => "サヨウナラ",

    // Pronouns
    "私" => "ワタシ",
    "僕" => "ボク",
    "俺" => "オレ",
    "君" => "キミ",
    "彼" => "カレ",
    "彼女" => "カノジョ",
    "誰" => "ダレ",
    "何" => "ナニ",
    "何処" => "ドコ",
    "此処" => "ココ",
    "其処" => "ソコ",
    "彼処" => "アソコ",

    // Common verbs
    "有る" => "アル",
    "ある" => "アル",
    "居る" => "イル",
    "いる" => "イル",
    "行く" => "イク",
    "いく" => "イク",
    "来る" => "クル",
    "くる" => "クル",
    "見る" => "ミル",
    "みる" => "ミル",
    "聞く" => "キク",
    "きく" => "キク",
    "話す" => "ハナス",
    "言う" => "イウ",
    "思う" => "オモウ",
    "知る" => "シル",
    "分かる" => "ワカル",
    "食べる" => "タベル",
    "飲む" => "ノム",
    "買う" => "カウ",
    "売る" => "ウル",
    "読む" => "ヨム",
    "書く" => "カク",
    "作る" => "ツクル",
    "使う" => "ツカウ",
    "働く" => "ハタラク",
    "遊ぶ" => "アソブ",
    "寝る" => "ネル",
    "起きる" => "オキル",
    "入る" => "ハイル",
    "出る" => "デル",
    "帰る" => "カエル",
    "待つ" => "マツ",
    "会う" => "アウ",
    "持つ" => "モツ",
    "置く" => "オク",
    "取る" => "トル",
    "立つ" => "タツ",
    "座る" => "スワル",
    "走る" => "ハシル",
    "歩く" => "アルク",
    "泳ぐ" => "オヨグ",
    "飛ぶ" => "トブ",
    "落ちる" => "オチル",
    "上がる" => "アガル",
    "下がる" => "サガル",
    "開ける" => "アケル",
    "閉める" => "シメル",
    "始める" => "ハジメル",
    "終わる" => "オワル",

    // Adjectives
    "大きい" => "オオキイ",
    "小さい" => "チイサイ",
    "高い" => "タカイ",
    "安い" => "ヤスイ",
    "新しい" => "アタラシイ",
    "古い" => "フルイ",
    "良い" => "ヨイ",
    "悪い" => "ワルイ",
    "長い" => "ナガイ",
    "短い" => "ミジカイ",
    "早い" => "ハヤイ",
    "遅い" => "オソイ",
    "暑い" => "アツイ",
    "寒い" => "サムイ",
    "暖かい" => "アタタカイ",
    "涼しい" => "スズシイ",
    "美しい" => "ウツクシイ",
    "楽しい" => "タノシイ",
    "嬉しい" => "ウレシイ",
    "悲しい" => "カナシイ",
    "面白い" => "オモシロイ",
    "難しい" => "ムズカシイ",
    "易しい" => "ヤサシイ",
    "優しい" => "ヤサシイ",
    "強い" => "ツヨイ",
    "弱い" => "ヨワイ",
    "多い" => "オオイ",
    "少ない" => "スクナイ",
    "広い" => "ヒロイ",
    "狭い" => "セマイ",
    "明るい" => "アカルイ",
    "暗い" => "クライ",
    "重い" => "オモイ",
    "軽い" => "カルイ",
    "甘い" => "アマイ",
    "辛い" => "カライ",

    // Nouns - Time
    "今日" => "キョウ",
    "明日" => "アシタ",
    "昨日" => "キノウ",
    "今" => "イマ",
    "後" => "アト",
    "前" => "マエ",
    "朝" => "アサ",
    "昼" => "ヒル",
    "夜" => "ヨル",
    "年" => "トシ",
    "月" => "ツキ",
    "日" => "ヒ",
    "週" => "シュウ",
    "時間" => "ジカン",
    "分" => "フン",
    "秒" => "ビョウ",

    // Nouns - People
    "人" => "ヒト",
    "男" => "オトコ",
    "女" => "オンナ",
    "子供" => "コドモ",
    "大人" => "オトナ",
    "友達" => "トモダチ",
    "家族" => "カゾク",
    "父" => "チチ",
    "母" => "ハハ",
    "兄" => "アニ",
    "姉" => "アネ",
    "弟" => "オトウト",
    "妹" => "イモウト",
    "息子" => "ムスコ",
    "娘" => "ムスメ",
    "先生" => "センセイ",
    "学生" => "ガクセイ",

    // Nouns - Places
    "家" => "イエ",
    "学校" => "ガッコウ",
    "会社" => "カイシャ",
    "店" => "ミセ",
    "駅" => "エキ",
    "病院" => "ビョウイン",
    "銀行" => "ギンコウ",
    "図書館" => "トショカン",
    "公園" => "コウエン",
    "空港" => "クウコウ",
    "国" => "クニ",
    "町" => "マチ",
    "村" => "ムラ",
    "山" => "ヤマ",
    "川" => "カワ",
    "海" => "ウミ",

    // Nouns - Things
    "物" => "モノ",
    "事" => "コト",
    "所" => "トコロ",
    "方" => "ホウ",
    "水" => "ミズ",
    "火" => "ヒ",
    "風" => "カゼ",
    "雨" => "アメ",
    "雪" => "ユキ",
    "空" => "ソラ",
    "星" => "ホシ",
    "花" => "ハナ",
    "木" => "キ",
    "本" => "ホン",
    "車" => "クルマ",
    "電車" => "デンシャ",
    "飛行機" => "ヒコウキ",
    "電話" => "デンワ",
    "手紙" => "テガミ",
    "写真" => "シャシン",
    "音楽" => "オンガク",
    "映画" => "エイガ",
    "新聞" => "シンブン",
    "雑誌" => "ザッシ",
    "食べ物" => "タベモノ",
    "飲み物" => "ノミモノ",
    "料理" => "リョウリ",
    "御飯" => "ゴハン",
    "お茶" => "オチャ",
    "酒" => "サケ",
    "金" => "カネ",
    "仕事" => "シゴト",
    "勉強" => "ベンキョウ",
    "旅行" => "リョコウ",
    "買い物" => "カイモノ",

    // Numbers
    "一" => "イチ",
    "二" => "ニ",
    "三" => "サン",
    "四" => "シ",
    "五" => "ゴ",
    "六" => "ロク",
    "七" => "シチ",
    "八" => "ハチ",
    "九" => "キュウ",
    "十" => "ジュウ",
    "百" => "ヒャク",
    "千" => "セン",
    "万" => "マン",
    "億" => "オク",

    // Direction/Position
    "上" => "ウエ",
    "下" => "シタ",
    "中" => "ナカ",
    "外" => "ソト",
    "右" => "ミギ",
    "左" => "ヒダリ",
    "北" => "キタ",
    "南" => "ミナミ",
    "東" => "ヒガシ",
    "西" => "ニシ",

    // Common expressions
    "元気" => "ゲンキ",
    "大丈夫" => "ダイジョウブ",
    "本当" => "ホントウ",
    "綺麗" => "キレイ",
    "可愛い" => "カワイイ",
    "格好いい" => "カッコイイ",
    "素晴らしい" => "スバラシイ",
    "凄い" => "スゴイ",

    // World
    "世界" => "セカイ",
    "日本" => "ニホン",
    "東京" => "トウキョウ",
    "大阪" => "オオサカ",
    "京都" => "キョウト",
};

lazy_static! {
    /// Single kanji readings for fallback
    pub static ref SINGLE_KANJI: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        // Common single kanji with on'yomi/kun'yomi readings
        m.insert('人', "ヒト");
        m.insert('日', "ヒ");
        m.insert('月', "ツキ");
        m.insert('年', "ネン");
        m.insert('時', "トキ");
        m.insert('間', "カン");
        m.insert('分', "ブン");
        m.insert('秒', "ビョウ");
        m.insert('上', "ウエ");
        m.insert('下', "シタ");
        m.insert('中', "ナカ");
        m.insert('外', "ソト");
        m.insert('前', "マエ");
        m.insert('後', "アト");
        m.insert('左', "ヒダリ");
        m.insert('右', "ミギ");
        m.insert('大', "オオ");
        m.insert('小', "チイ");
        m.insert('長', "ナガ");
        m.insert('短', "ミジカ");
        m.insert('高', "タカ");
        m.insert('安', "ヤス");
        m.insert('新', "アタラ");
        m.insert('古', "フル");
        m.insert('良', "ヨ");
        m.insert('悪', "ワル");
        m.insert('水', "ミズ");
        m.insert('火', "ヒ");
        m.insert('木', "キ");
        m.insert('金', "カネ");
        m.insert('土', "ツチ");
        m.insert('山', "ヤマ");
        m.insert('川', "カワ");
        m.insert('海', "ウミ");
        m.insert('空', "ソラ");
        m.insert('雨', "アメ");
        m.insert('雪', "ユキ");
        m.insert('風', "カゼ");
        m.insert('花', "ハナ");
        m.insert('草', "クサ");
        m.insert('虫', "ムシ");
        m.insert('鳥', "トリ");
        m.insert('魚', "サカナ");
        m.insert('犬', "イヌ");
        m.insert('猫', "ネコ");
        m.insert('馬', "ウマ");
        m.insert('牛', "ウシ");
        m.insert('羊', "ヒツジ");
        m.insert('豚', "ブタ");
        m.insert('父', "チチ");
        m.insert('母', "ハハ");
        m.insert('兄', "アニ");
        m.insert('姉', "アネ");
        m.insert('弟', "オトウト");
        m.insert('妹', "イモウト");
        m.insert('子', "コ");
        m.insert('男', "オトコ");
        m.insert('女', "オンナ");
        m.insert('手', "テ");
        m.insert('足', "アシ");
        m.insert('目', "メ");
        m.insert('耳', "ミミ");
        m.insert('口', "クチ");
        m.insert('鼻', "ハナ");
        m.insert('顔', "カオ");
        m.insert('頭', "アタマ");
        m.insert('心', "ココロ");
        m.insert('声', "コエ");
        m.insert('力', "チカラ");
        m.insert('気', "キ");
        m.insert('本', "ホン");
        m.insert('車', "クルマ");
        m.insert('船', "フネ");
        m.insert('店', "ミセ");
        m.insert('家', "イエ");
        m.insert('国', "クニ");
        m.insert('町', "マチ");
        m.insert('村', "ムラ");
        m.insert('道', "ミチ");
        m.insert('駅', "エキ");
        m.insert('門', "モン");
        m.insert('窓', "マド");
        m.insert('戸', "ト");
        m.insert('石', "イシ");
        m.insert('糸', "イト");
        m.insert('紙', "カミ");
        m.insert('音', "オト");
        m.insert('色', "イロ");
        m.insert('白', "シロ");
        m.insert('黒', "クロ");
        m.insert('赤', "アカ");
        m.insert('青', "アオ");
        m.insert('緑', "ミドリ");
        m.insert('黄', "キ");
        m.insert('朝', "アサ");
        m.insert('昼', "ヒル");
        m.insert('夜', "ヨル");
        m.insert('夕', "ユウ");
        m.insert('春', "ハル");
        m.insert('夏', "ナツ");
        m.insert('秋', "アキ");
        m.insert('冬', "フユ");
        m.insert('今', "イマ");
        m.insert('昔', "ムカシ");
        m.insert('北', "キタ");
        m.insert('南', "ミナミ");
        m.insert('東', "ヒガシ");
        m.insert('西', "ニシ");
        m.insert('私', "ワタシ");
        m.insert('僕', "ボク");
        m.insert('俺', "オレ");
        m.insert('君', "キミ");
        m.insert('彼', "カレ");
        m.insert('誰', "ダレ");
        m.insert('何', "ナニ");
        m.insert('一', "イチ");
        m.insert('二', "ニ");
        m.insert('三', "サン");
        m.insert('四', "シ");
        m.insert('五', "ゴ");
        m.insert('六', "ロク");
        m.insert('七', "シチ");
        m.insert('八', "ハチ");
        m.insert('九', "キュウ");
        m.insert('十', "ジュウ");
        m.insert('百', "ヒャク");
        m.insert('千', "セン");
        m.insert('万', "マン");
        m
    };
}

/// Check if a character is a kanji
pub fn is_kanji(c: char) -> bool {
    // CJK Unified Ideographs: U+4E00-U+9FFF
    // CJK Extension A: U+3400-U+4DBF
    matches!(c, '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}')
}

/// Look up reading for a word
pub fn get_reading(word: &str) -> Option<&'static str> {
    READINGS.get(word).copied()
}

/// Get reading for single kanji
pub fn get_single_kanji_reading(c: char) -> Option<&'static str> {
    SINGLE_KANJI.get(&c).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readings() {
        assert_eq!(get_reading("今日"), Some("キョウ"));
        assert_eq!(get_reading("ありがとう"), Some("アリガトウ"));
        assert_eq!(get_reading("世界"), Some("セカイ"));
    }

    #[test]
    fn test_single_kanji() {
        assert_eq!(get_single_kanji_reading('人'), Some("ヒト"));
        assert_eq!(get_single_kanji_reading('山'), Some("ヤマ"));
    }

    #[test]
    fn test_is_kanji() {
        assert!(is_kanji('人'));
        assert!(is_kanji('日'));
        assert!(!is_kanji('あ'));
        assert!(!is_kanji('ア'));
        assert!(!is_kanji('a'));
    }
}
