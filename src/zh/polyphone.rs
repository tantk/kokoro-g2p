//! Polyphone resolution for Chinese characters
//!
//! Many Chinese characters have multiple pronunciations (polyphones/多音字).
//! This module handles disambiguation based on:
//! 1. Phrase-based lookup (highest priority)
//! 2. POS-based disambiguation
//! 3. Frequency-based default

use once_cell::sync::Lazy;
use phf::phf_map;
use std::collections::HashMap;

/// Phrase-based polyphone mappings (word -> pinyin with tones)
/// These are common multi-character words where pronunciation is context-dependent
static PHRASE_PINYIN: phf::Map<&'static str, &'static str> = phf_map! {
    // 行 (xíng = walk/travel, háng = row/profession)
    "行走" => "xing2 zou3",
    "行人" => "xing2 ren2",
    "行动" => "xing2 dong4",
    "行为" => "xing2 wei2",
    "执行" => "zhi2 xing2",
    "进行" => "jin4 xing2",
    "举行" => "ju3 xing2",
    "银行" => "yin2 hang2",
    "行业" => "hang2 ye4",
    "行列" => "hang2 lie4",
    "一行" => "yi4 hang2",
    "同行" => "tong2 hang2",

    // 了 (le = particle, liǎo = understand/finish)
    "了解" => "liao3 jie3",
    "了不起" => "liao3 bu4 qi3",
    "了结" => "liao3 jie2",
    "明了" => "ming2 liao3",
    "受不了" => "shou4 bu4 liao3",

    // 得 (de = particle, dé = obtain, děi = must)
    "得到" => "de2 dao4",
    "取得" => "qu3 de2",
    "获得" => "huo4 de2",
    "得知" => "de2 zhi1",
    "觉得" => "jue2 de5",

    // 地 (dì = earth/ground, de = particle)
    "地方" => "di4 fang1",
    "地球" => "di4 qiu2",
    "地区" => "di4 qu1",
    "土地" => "tu3 di4",

    // 的 (de = particle, dí = target, dì = indeed)
    "目的" => "mu4 di4",
    "的确" => "di2 que4",

    // 还 (hái = still, huán = return)
    "还是" => "hai2 shi4",
    "还有" => "hai2 you3",
    "还要" => "hai2 yao4",
    "归还" => "gui1 huan2",
    "偿还" => "chang2 huan2",
    "还原" => "huan2 yuan2",

    // 长 (cháng = long, zhǎng = grow/chief)
    "长城" => "chang2 cheng2",
    "长度" => "chang2 du4",
    "长期" => "chang2 qi1",
    "长久" => "chang2 jiu3",
    "成长" => "cheng2 zhang3",
    "生长" => "sheng1 zhang3",
    "增长" => "zeng1 zhang3",
    "校长" => "xiao4 zhang3",
    "部长" => "bu4 zhang3",
    "市长" => "shi4 zhang3",

    // 重 (zhòng = heavy, chóng = repeat)
    "重要" => "zhong4 yao4",
    "重点" => "zhong4 dian3",
    "重视" => "zhong4 shi4",
    "重量" => "zhong4 liang4",
    "重复" => "chong2 fu4",
    "重新" => "chong2 xin1",

    // 乐 (lè = happy, yuè = music)
    "快乐" => "kuai4 le4",
    "欢乐" => "huan1 le4",
    "音乐" => "yin1 yue4",
    "乐器" => "yue4 qi4",

    // 教 (jiào = teach/religion, jiāo = instruct)
    "教育" => "jiao4 yu4",
    "教学" => "jiao4 xue2",
    "教室" => "jiao4 shi4",
    "宗教" => "zong1 jiao4",
    "教书" => "jiao1 shu1",

    // 数 (shù = number, shǔ = count)
    "数字" => "shu4 zi4",
    "数学" => "shu4 xue2",
    "数量" => "shu4 liang4",
    "数据" => "shu4 ju4",

    // 空 (kōng = empty/sky, kòng = free time)
    "空气" => "kong1 qi4",
    "空间" => "kong1 jian1",
    "天空" => "tian1 kong1",
    "空调" => "kong1 tiao2",
    "有空" => "you3 kong4",

    // 差 (chà = differ, chāi = send, chā = difference)
    "差不多" => "cha4 bu5 duo1",
    "差别" => "cha1 bie2",
    "出差" => "chu1 chai1",

    // 难 (nán = difficult, nàn = disaster)
    "困难" => "kun4 nan2",
    "难过" => "nan2 guo4",
    "难题" => "nan2 ti2",
    "灾难" => "zai1 nan4",
    "难民" => "nan4 min2",

    // 便 (biàn = convenient, pián = cheap)
    "方便" => "fang1 bian4",
    "便利" => "bian4 li4",
    "便宜" => "pian2 yi5",

    // 兴 (xīng = prosper, xìng = interest)
    "高兴" => "gao1 xing4",
    "兴趣" => "xing4 qu4",
    "兴奋" => "xing1 fen4",
    "复兴" => "fu4 xing1",

    // 朝 (cháo = dynasty/toward, zhāo = morning)
    "朝代" => "chao2 dai4",
    "朝向" => "chao2 xiang4",

    // 更 (gèng = more, gēng = change)
    "更加" => "geng4 jia1",
    "更好" => "geng4 hao3",
    "变更" => "bian4 geng1",

    // 处 (chù = place, chǔ = handle)
    "处理" => "chu3 li3",
    "到处" => "dao4 chu4",
    "处于" => "chu3 yu2",

    // 调 (diào = tone/transfer, tiáo = adjust)
    "调查" => "diao4 cha2",
    "调整" => "tiao2 zheng3",
    "调节" => "tiao2 jie2",
    "声调" => "sheng1 diao4",

    // 藏 (cáng = hide, zàng = Tibet/storage)
    "西藏" => "xi1 zang4",
    "收藏" => "shou1 cang2",
    "隐藏" => "yin3 cang2",

    // 称 (chēng = call, chèn = fit)
    "称呼" => "cheng1 hu1",
    "称为" => "cheng1 wei2",
    "名称" => "ming2 cheng1",
    "对称" => "dui4 chen4",
    "匀称" => "yun2 chen4",

    // 少 (shǎo = few, shào = young)
    "多少" => "duo1 shao3",
    "减少" => "jian3 shao3",
    "少年" => "shao4 nian2",
    "少数" => "shao3 shu4",

    // 省 (shěng = province/save, xǐng = reflect)
    "省份" => "sheng3 fen4",
    "节省" => "jie2 sheng3",
    "反省" => "fan3 xing3",

    // 相 (xiāng = mutual, xiàng = appearance)
    "相信" => "xiang1 xin4",
    "相同" => "xiang1 tong2",
    "相关" => "xiang1 guan1",
    "照相" => "zhao4 xiang4",
    "相机" => "xiang4 ji1",
    "真相" => "zhen1 xiang4",

    // 好 (hǎo = good, hào = like)
    "你好" => "ni3 hao3",
    "好的" => "hao3 de5",
    "爱好" => "ai4 hao4",
    "好奇" => "hao4 qi2",

    // 中 (zhōng = middle, zhòng = hit)
    "中国" => "zhong1 guo2",
    "中间" => "zhong1 jian1",
    "中心" => "zhong1 xin1",
    "命中" => "ming4 zhong4",

    // 没 (méi = not have, mò = sink)
    "没有" => "mei2 you3",
    "没关系" => "mei2 guan1 xi5",
    "淹没" => "yan1 mo4",
    "沉没" => "chen2 mo4",

    // Common phrases
    "什么" => "shen2 me5",
    "怎么" => "zen3 me5",
    "那么" => "na4 me5",
    "这么" => "zhe4 me5",
    "为什么" => "wei4 shen2 me5",
    "喜欢" => "xi3 huan1",
    "知道" => "zhi1 dao4",
    "可以" => "ke3 yi3",
    "应该" => "ying1 gai1",
    "需要" => "xu1 yao4",
    "已经" => "yi3 jing1",
    "虽然" => "sui1 ran2",
    "但是" => "dan4 shi4",
    "因为" => "yin1 wei4",
    "所以" => "suo3 yi3",
    "如果" => "ru2 guo3",
    "这样" => "zhe4 yang4",
    "那样" => "na4 yang4",
    "非常" => "fei1 chang2",
    "特别" => "te4 bie2",
    "比较" => "bi3 jiao4",
    "可能" => "ke3 neng2",
    "必须" => "bi4 xu1",
    "其实" => "qi2 shi2",
    "现在" => "xian4 zai4",
    "以后" => "yi3 hou4",
    "以前" => "yi3 qian2",
};

/// POS-based default pinyin for polyphones
/// Maps (character, POS tag prefix) to pinyin
static POS_BASED_PINYIN: Lazy<HashMap<(char, &'static str), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // 行: xíng for verbs, háng for nouns
    m.insert(('行', "v"), "xing2");
    m.insert(('行', "n"), "hang2");

    // 了: liǎo for verbs, le for particles
    m.insert(('了', "v"), "liao3");
    m.insert(('了', "u"), "le5");

    // 得: dé for verbs, de for particles, děi for auxiliary
    m.insert(('得', "v"), "de2");
    m.insert(('得', "u"), "de5");

    // 地: dì for nouns, de for particles
    m.insert(('地', "n"), "di4");
    m.insert(('地', "u"), "de5");

    // 还: hái for adverbs, huán for verbs
    m.insert(('还', "d"), "hai2");
    m.insert(('还', "v"), "huan2");

    // 长: cháng for adjectives, zhǎng for verbs/nouns (chief)
    m.insert(('长', "a"), "chang2");
    m.insert(('长', "v"), "zhang3");
    m.insert(('长', "n"), "zhang3");

    // 重: zhòng for adjectives, chóng for adverbs/verbs
    m.insert(('重', "a"), "zhong4");
    m.insert(('重', "d"), "chong2");

    // 乐: lè for adjectives, yuè for nouns
    m.insert(('乐', "a"), "le4");
    m.insert(('乐', "n"), "yue4");

    // 教: jiào for nouns, jiāo for verbs
    m.insert(('教', "n"), "jiao4");
    m.insert(('教', "v"), "jiao1");

    // 数: shù for nouns, shǔ for verbs
    m.insert(('数', "n"), "shu4");
    m.insert(('数', "v"), "shu3");

    m
});

/// Default pinyin for characters (most common reading)
static DEFAULT_PINYIN: phf::Map<char, &'static str> = phf_map! {
    '行' => "xing2",
    '了' => "le5",
    '得' => "de5",
    '地' => "di4",
    '的' => "de5",
    '还' => "hai2",
    '长' => "chang2",
    '重' => "zhong4",
    '乐' => "le4",
    '教' => "jiao4",
    '数' => "shu4",
    '空' => "kong1",
    '差' => "cha4",
    '难' => "nan2",
    '便' => "bian4",
    '兴' => "xing4",
    '朝' => "chao2",
    '更' => "geng4",
    '处' => "chu4",
    '调' => "diao4",
    '藏' => "cang2",
    '称' => "cheng1",
    '少' => "shao3",
    '省' => "sheng3",
    '相' => "xiang1",
    '好' => "hao3",
    '中' => "zhong1",
    '没' => "mei2",
};

/// Look up a phrase in the polyphone dictionary
pub fn lookup_phrase(phrase: &str) -> Option<&'static str> {
    PHRASE_PINYIN.get(phrase).copied()
}

/// Look up a character's pinyin based on POS tag
pub fn lookup_with_pos(c: char, pos: &str) -> Option<&'static str> {
    // Try exact POS tag first
    if let Some(&pinyin) = POS_BASED_PINYIN.get(&(c, pos)) {
        return Some(pinyin);
    }

    // Try POS tag prefix (first character)
    if !pos.is_empty() {
        let pos_prefix = &pos[..1];
        if let Some(&pinyin) = POS_BASED_PINYIN.get(&(c, pos_prefix)) {
            return Some(pinyin);
        }
    }

    None
}

/// Get the default pinyin for a character
pub fn get_default_pinyin(c: char) -> Option<&'static str> {
    DEFAULT_PINYIN.get(&c).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phrase_lookup() {
        assert_eq!(lookup_phrase("银行"), Some("yin2 hang2"));
        assert_eq!(lookup_phrase("行走"), Some("xing2 zou3"));
        assert_eq!(lookup_phrase("不存在"), None);
    }

    #[test]
    fn test_pos_lookup() {
        // 行 as verb -> xíng
        assert_eq!(lookup_with_pos('行', "v"), Some("xing2"));
        // 行 as noun -> háng
        assert_eq!(lookup_with_pos('行', "n"), Some("hang2"));
    }

    #[test]
    fn test_default_pinyin() {
        assert_eq!(get_default_pinyin('行'), Some("xing2"));
        assert_eq!(get_default_pinyin('了'), Some("le5"));
    }
}
