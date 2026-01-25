//! Tone sandhi rules for Mandarin Chinese
//!
//! Implements the standard Mandarin tone change rules:
//! 1. Third tone sandhi: 3-3 → 2-3
//! 2. 一 (yī) tone changes:
//!    - Before 4th tone: yī → yí (1→2)
//!    - Before 1st/2nd/3rd tone: yī → yì (1→4)
//! 3. 不 (bù) tone change:
//!    - Before 4th tone: bù → bú (4→2)

use super::pinyin::PinyinSyllable;

/// Apply all tone sandhi rules to a sequence of pinyin syllables
pub fn apply_tone_sandhi(syllables: &[PinyinSyllable]) -> Vec<PinyinSyllable> {
    if syllables.is_empty() {
        return Vec::new();
    }

    let mut result = syllables.to_vec();

    // Apply rules in order
    apply_yi_sandhi(&mut result);
    apply_bu_sandhi(&mut result);
    apply_third_tone_sandhi(&mut result);

    result
}

/// Apply 一 (yī) tone sandhi
/// - Before 4th tone: yī → yí (tone 2)
/// - Before other tones: yī → yì (tone 4)
/// - In isolation or at end: remains yī (tone 1)
fn apply_yi_sandhi(syllables: &mut [PinyinSyllable]) {
    for i in 0..syllables.len() {
        if syllables[i].syllable == "yi" && syllables[i].tone == 1 {
            // Check if this is 一 (not other yi1 syllables)
            // This is a simplification; in practice we'd check the original character
            if i + 1 < syllables.len() {
                let next_tone = syllables[i + 1].tone;
                match next_tone {
                    4 => syllables[i].tone = 2, // Before 4th: yī → yí
                    1 | 2 | 3 => syllables[i].tone = 4, // Before others: yī → yì
                    _ => {} // Neutral tone: usually stays yī
                }
            }
            // At end of phrase: stays yī (tone 1)
        }
    }
}

/// Apply 不 (bù) tone sandhi
/// - Before 4th tone: bù → bú (tone 2)
fn apply_bu_sandhi(syllables: &mut [PinyinSyllable]) {
    for i in 0..syllables.len() {
        if syllables[i].syllable == "bu" && syllables[i].tone == 4 {
            if i + 1 < syllables.len() && syllables[i + 1].tone == 4 {
                syllables[i].tone = 2; // Before 4th tone: bù → bú
            }
        }
    }
}

/// Apply third tone sandhi (3-3 → 2-3)
/// When two or more third tones appear in sequence, all but the last change to tone 2
fn apply_third_tone_sandhi(syllables: &mut [PinyinSyllable]) {
    if syllables.len() < 2 {
        return;
    }

    // Find sequences of third tones and change all but the last to tone 2
    let mut i = 0;
    while i < syllables.len() {
        if syllables[i].tone == 3 {
            // Find the extent of the third-tone sequence
            let mut j = i;
            while j + 1 < syllables.len() && syllables[j + 1].tone == 3 {
                j += 1;
            }

            // If we have a sequence (more than one third tone)
            if j > i {
                // Change all but the last to tone 2
                for k in i..j {
                    syllables[k].tone = 2;
                }
            }

            i = j + 1;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_syllables(specs: &[(&str, u8)]) -> Vec<PinyinSyllable> {
        specs
            .iter()
            .map(|(s, t)| PinyinSyllable::new(s, *t))
            .collect()
    }

    #[test]
    fn test_third_tone_sandhi() {
        // 你好 (nǐ hǎo) → (ní hǎo)
        let input = make_syllables(&[("ni", 3), ("hao", 3)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 2); // ni3 → ni2
        assert_eq!(result[1].tone, 3); // hao3 stays
    }

    #[test]
    fn test_third_tone_sandhi_three() {
        // 你好吗 (nǐ hǎo ma) - "好" is followed by neutral tone ma
        // But if all three were tone 3: nǐ hǎo mǎ → ní hǎo mǎ
        // Actually for three in a row: 222-3 or 2-2-3 depending on grouping
        let input = make_syllables(&[("ni", 3), ("hao", 3), ("ma", 3)]);
        let result = apply_tone_sandhi(&input);
        // All but last should change
        assert_eq!(result[0].tone, 2);
        assert_eq!(result[1].tone, 2);
        assert_eq!(result[2].tone, 3);
    }

    #[test]
    fn test_yi_before_fourth() {
        // 一个 (yī gè) → (yí gè)
        let input = make_syllables(&[("yi", 1), ("ge", 4)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 2); // yi1 → yi2 before tone 4
    }

    #[test]
    fn test_yi_before_first() {
        // 一天 (yī tiān) → (yì tiān)
        let input = make_syllables(&[("yi", 1), ("tian", 1)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 4); // yi1 → yi4 before tone 1
    }

    #[test]
    fn test_yi_before_second() {
        // 一年 (yī nián) → (yì nián)
        let input = make_syllables(&[("yi", 1), ("nian", 2)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 4); // yi1 → yi4 before tone 2
    }

    #[test]
    fn test_yi_before_third() {
        // 一起 (yī qǐ) → (yì qǐ)
        let input = make_syllables(&[("yi", 1), ("qi", 3)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 4); // yi1 → yi4 before tone 3
    }

    #[test]
    fn test_bu_before_fourth() {
        // 不是 (bù shì) → (bú shì)
        let input = make_syllables(&[("bu", 4), ("shi", 4)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 2); // bu4 → bu2 before tone 4
    }

    #[test]
    fn test_bu_before_other() {
        // 不好 (bù hǎo) - stays bù
        let input = make_syllables(&[("bu", 4), ("hao", 3)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 4); // bu4 stays before tone 3
    }

    #[test]
    fn test_no_change_needed() {
        // 你好世界 with mixed tones
        let input = make_syllables(&[("ni", 3), ("hao", 3), ("shi", 4), ("jie", 4)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 2); // Third-tone sandhi
        assert_eq!(result[1].tone, 3);
        assert_eq!(result[2].tone, 4);
        assert_eq!(result[3].tone, 4);
    }

    #[test]
    fn test_empty_input() {
        let input: Vec<PinyinSyllable> = vec![];
        let result = apply_tone_sandhi(&input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_syllable() {
        let input = make_syllables(&[("ni", 3)]);
        let result = apply_tone_sandhi(&input);
        assert_eq!(result[0].tone, 3); // No change for single syllable
    }
}
