//! Lexicon module for dictionary-based phoneme lookup
//!
//! Implements the cascading dictionary lookup strategy:
//! 1. Gold dictionary (highest quality)
//! 2. Silver dictionary (fallback)
//! 3. Stemming rules for -s, -ed, -ing suffixes

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Phoneme entry that can be either a simple string or tag-dependent
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PhonemeEntry {
    Simple(String),
    Tagged(HashMap<String, Option<String>>),
}

impl PhonemeEntry {
    /// Get the phoneme string for a given POS tag
    pub fn get(&self, tag: Option<&str>) -> Option<&str> {
        match self {
            PhonemeEntry::Simple(s) => Some(s.as_str()),
            PhonemeEntry::Tagged(map) => {
                // Try exact tag match first
                if let Some(tag) = tag {
                    if let Some(Some(val)) = map.get(tag) {
                        return Some(val.as_str());
                    }
                    // Try parent tag
                    let parent = get_parent_tag(tag);
                    if parent != tag {
                        if let Some(Some(val)) = map.get(parent) {
                            return Some(val.as_str());
                        }
                    }
                }
                // Fall back to DEFAULT
                map.get("DEFAULT")
                    .and_then(|v| v.as_ref())
                    .map(|s| s.as_str())
            }
        }
    }
}

/// Map POS tags to their parent categories
fn get_parent_tag(tag: &str) -> &str {
    if tag.starts_with("VB") {
        "VERB"
    } else if tag.starts_with("NN") {
        "NOUN"
    } else if tag.starts_with("ADV") || tag.starts_with("RB") {
        "ADV"
    } else if tag.starts_with("ADJ") || tag.starts_with("JJ") {
        "ADJ"
    } else {
        tag
    }
}

/// Stress markers
const PRIMARY_STRESS: char = 'ˈ';
const SECONDARY_STRESS: char = 'ˌ';
const PRIMARY_STRESS_STR: &str = "ˈ";
const SECONDARY_STRESS_STR: &str = "ˌ";

/// Vowels for stress placement
const VOWELS: &str = "AIOQWYaiuæɑɒɔəɛɜɪʊʌᵻ";

/// Characters that indicate word ending sounds
const VOICELESS_ENDINGS: &str = "ptkfθʃsʧ";
const SIBILANT_ENDINGS: &str = "szʃʒʧʤ";

/// Dictionary type
type Dictionary = HashMap<String, PhonemeEntry>;

/// Load a dictionary from JSON string
fn load_dictionary(json: &str) -> Dictionary {
    serde_json::from_str(json).unwrap_or_else(|e| {
        log::error!("Failed to parse dictionary: {}", e);
        HashMap::new()
    })
}

/// Grow dictionary with case variants
fn grow_dictionary(dict: &mut Dictionary) {
    let additions: Vec<(String, PhonemeEntry)> = dict
        .iter()
        .filter(|(k, _)| k.len() >= 2)
        .filter_map(|(k, v)| {
            if k == &k.to_lowercase() {
                // lowercase -> Capitalized
                let cap = capitalize(k);
                if &cap != k && !dict.contains_key(&cap) {
                    Some((cap, v.clone()))
                } else {
                    None
                }
            } else if k == &capitalize(k) {
                // Capitalized -> lowercase
                let lower = k.to_lowercase();
                if &lower != k && !dict.contains_key(&lower) {
                    Some((lower, v.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    for (k, v) in additions {
        dict.insert(k, v);
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// The Lexicon handles word to phoneme conversion
pub struct Lexicon {
    british: bool,
    gold: Dictionary,
    silver: Dictionary,
}

// Embed dictionaries at compile time
static US_GOLD_JSON: &str = include_str!("../dictionaries/us_gold.json");
static US_SILVER_JSON: &str = include_str!("../dictionaries/us_silver.json");
static GB_GOLD_JSON: &str = include_str!("../dictionaries/gb_gold.json");
static GB_SILVER_JSON: &str = include_str!("../dictionaries/gb_silver.json");

// Pre-loaded US dictionaries
static US_GOLD: Lazy<Dictionary> = Lazy::new(|| {
    let mut d = load_dictionary(US_GOLD_JSON);
    grow_dictionary(&mut d);
    d
});

static US_SILVER: Lazy<Dictionary> = Lazy::new(|| {
    let mut d = load_dictionary(US_SILVER_JSON);
    grow_dictionary(&mut d);
    d
});

// Pre-loaded GB dictionaries
static GB_GOLD: Lazy<Dictionary> = Lazy::new(|| {
    let mut d = load_dictionary(GB_GOLD_JSON);
    grow_dictionary(&mut d);
    d
});

static GB_SILVER: Lazy<Dictionary> = Lazy::new(|| {
    let mut d = load_dictionary(GB_SILVER_JSON);
    grow_dictionary(&mut d);
    d
});

impl Lexicon {
    /// Create a new Lexicon for American or British English
    pub fn new(british: bool) -> Self {
        if british {
            Self {
                british,
                gold: GB_GOLD.clone(),
                silver: GB_SILVER.clone(),
            }
        } else {
            Self {
                british,
                gold: US_GOLD.clone(),
                silver: US_SILVER.clone(),
            }
        }
    }

    /// Create a lexicon using static references (more memory efficient)
    pub fn new_static(british: bool) -> LexiconRef {
        LexiconRef {
            british,
            gold: if british { &GB_GOLD } else { &US_GOLD },
            silver: if british { &GB_SILVER } else { &US_SILVER },
        }
    }

    /// Check if a word is in the lexicon
    pub fn contains(&self, word: &str) -> bool {
        self.gold.contains_key(word) || self.silver.contains_key(word)
    }

    /// Look up a word's phonemes
    pub fn lookup(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        // Try gold dictionary first
        if let Some(entry) = self.gold.get(word) {
            if let Some(ps) = entry.get(tag) {
                return Some((ps.to_string(), 4));
            }
        }
        // Try silver dictionary
        if let Some(entry) = self.silver.get(word) {
            if let Some(ps) = entry.get(tag) {
                return Some((ps.to_string(), 3));
            }
        }
        None
    }

    /// Get phonemes for a word, trying various strategies
    pub fn get_word(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        let word_lower = word.to_lowercase();

        // Direct lookup
        if let Some(result) = self.lookup(word, tag) {
            return Some(result);
        }

        // Try lowercase for uppercase words
        if word != word_lower && word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
            if let Some(result) = self.lookup(&word_lower, tag) {
                return Some(result);
            }
        }

        // Try stemming
        if let Some(result) = self.try_stem(word, tag) {
            return Some(result);
        }

        None
    }

    /// Try to find word by stemming suffixes
    fn try_stem(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        // Try -s suffix
        if let Some(result) = self.stem_s(word, tag) {
            return Some(result);
        }
        // Try -ed suffix
        if let Some(result) = self.stem_ed(word, tag) {
            return Some(result);
        }
        // Try -ing suffix
        if let Some(result) = self.stem_ing(word, tag) {
            return Some(result);
        }
        None
    }

    /// Handle -s suffix (plurals, third person)
    fn stem_s(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        if word.len() < 3 || !word.ends_with('s') {
            return None;
        }

        let stem = if !word.ends_with("ss") && self.contains(&word[..word.len() - 1]) {
            &word[..word.len() - 1]
        } else if (word.ends_with("'s") || (word.len() > 4 && word.ends_with("es") && !word.ends_with("ies")))
            && self.contains(&word[..word.len() - 2])
        {
            &word[..word.len() - 2]
        } else if word.len() > 4 && word.ends_with("ies") {
            let base = format!("{}y", &word[..word.len() - 3]);
            if self.contains(&base) {
                return self.lookup(&base, tag).map(|(ps, rating)| (self.apply_s(&ps), rating));
            }
            return None;
        } else {
            return None;
        };

        self.lookup(stem, tag).map(|(ps, rating)| (self.apply_s(&ps), rating))
    }

    /// Apply -s suffix phoneme rules
    fn apply_s(&self, stem: &str) -> String {
        if stem.is_empty() {
            return String::new();
        }

        let last = stem.chars().last().unwrap();
        if VOICELESS_ENDINGS.contains(last) && !SIBILANT_ENDINGS.contains(last) {
            format!("{}s", stem)
        } else if SIBILANT_ENDINGS.contains(last) {
            let schwa = if self.british { "ɪ" } else { "ᵻ" };
            format!("{}{}z", stem, schwa)
        } else {
            format!("{}z", stem)
        }
    }

    /// Handle -ed suffix (past tense)
    fn stem_ed(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        if word.len() < 4 || !word.ends_with('d') {
            return None;
        }

        let stem = if !word.ends_with("dd") && self.contains(&word[..word.len() - 1]) {
            &word[..word.len() - 1]
        } else if word.len() > 4
            && word.ends_with("ed")
            && !word.ends_with("eed")
            && self.contains(&word[..word.len() - 2])
        {
            &word[..word.len() - 2]
        } else {
            return None;
        };

        self.lookup(stem, tag).map(|(ps, rating)| (self.apply_ed(&ps), rating))
    }

    /// Apply -ed suffix phoneme rules
    fn apply_ed(&self, stem: &str) -> String {
        if stem.is_empty() {
            return String::new();
        }

        let last = stem.chars().last().unwrap();
        if "pkfθʃsʧ".contains(last) {
            format!("{}t", stem)
        } else if last == 'd' {
            let schwa = if self.british { "ɪ" } else { "ᵻ" };
            format!("{}{}d", stem, schwa)
        } else if last != 't' {
            format!("{}d", stem)
        } else if self.british || stem.len() < 2 {
            format!("{}ɪd", stem)
        } else {
            // Check for American flap T
            let chars: Vec<char> = stem.chars().collect();
            let second_last = chars[chars.len() - 2];
            if "AIOWYiuæɑəɛɪɹʊʌ".contains(second_last) {
                let base = &stem[..stem.len() - 1];
                format!("{}ɾᵻd", base)
            } else {
                format!("{}ᵻd", stem)
            }
        }
    }

    /// Handle -ing suffix
    fn stem_ing(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        if word.len() < 5 || !word.ends_with("ing") {
            return None;
        }

        let base = &word[..word.len() - 3];
        let stem = if word.len() > 5 && self.contains(base) {
            base.to_string()
        } else if self.contains(&format!("{}e", base)) {
            format!("{}e", base)
        } else if word.len() > 5 && is_doubled_consonant(word) && self.contains(&word[..word.len() - 4]) {
            word[..word.len() - 4].to_string()
        } else {
            return None;
        };

        self.lookup(&stem, tag).map(|(ps, rating)| (self.apply_ing(&ps), rating))
    }

    /// Apply -ing suffix phoneme rules
    fn apply_ing(&self, stem: &str) -> String {
        if stem.is_empty() {
            return String::new();
        }

        if self.british {
            let last = stem.chars().last().unwrap();
            if "əː".contains(last) {
                return String::new(); // Cannot apply
            }
        } else if stem.len() > 1 {
            let chars: Vec<char> = stem.chars().collect();
            let last = chars[chars.len() - 1];
            let second_last = chars[chars.len() - 2];
            if last == 't' && "AIOWYiuæɑəɛɪɹʊʌ".contains(second_last) {
                let base = &stem[..stem.len() - 1];
                return format!("{}ɾɪŋ", base);
            }
        }
        format!("{}ɪŋ", stem)
    }
}

/// Check if word ends with doubled consonant before -ing
fn is_doubled_consonant(word: &str) -> bool {
    if word.ends_with("cking") {
        return true;
    }
    if !word.ends_with("ing") || word.len() < 5 {
        return false;
    }
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();
    // Check if the two chars before "ing" are the same consonant
    let c1 = chars[len - 5];
    let c2 = chars[len - 4];
    c1 == c2 && matches!(c1, 'b' | 'c' | 'd' | 'g' | 'k' | 'l' | 'm' | 'n' | 'p' | 'r' | 's' | 't' | 'v' | 'x' | 'z')
}

/// Reference-based lexicon (more memory efficient)
pub struct LexiconRef {
    british: bool,
    gold: &'static Dictionary,
    silver: &'static Dictionary,
}

impl LexiconRef {
    /// Look up a word's phonemes
    pub fn lookup(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        if let Some(entry) = self.gold.get(word) {
            if let Some(ps) = entry.get(tag) {
                return Some((ps.to_string(), 4));
            }
        }
        if let Some(entry) = self.silver.get(word) {
            if let Some(ps) = entry.get(tag) {
                return Some((ps.to_string(), 3));
            }
        }
        None
    }

    /// Check if a word is in the lexicon
    pub fn contains(&self, word: &str) -> bool {
        self.gold.contains_key(word) || self.silver.contains_key(word)
    }

    /// Get phonemes for a word
    pub fn get_word(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        let word_lower = word.to_lowercase();

        if let Some(result) = self.lookup(word, tag) {
            return Some(result);
        }

        if word != word_lower {
            if let Some(result) = self.lookup(&word_lower, tag) {
                return Some(result);
            }
        }

        self.try_stem(word, tag)
    }

    fn try_stem(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        self.stem_s(word, tag)
            .or_else(|| self.stem_ed(word, tag))
            .or_else(|| self.stem_ing(word, tag))
    }

    fn stem_s(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        if word.len() < 3 || !word.ends_with('s') {
            return None;
        }

        let stem = if !word.ends_with("ss") && self.contains(&word[..word.len() - 1]) {
            word[..word.len() - 1].to_string()
        } else if (word.ends_with("'s") || (word.len() > 4 && word.ends_with("es") && !word.ends_with("ies")))
            && self.contains(&word[..word.len() - 2])
        {
            word[..word.len() - 2].to_string()
        } else if word.len() > 4 && word.ends_with("ies") {
            let base = format!("{}y", &word[..word.len() - 3]);
            if self.contains(&base) {
                return self.lookup(&base, tag).map(|(ps, rating)| (apply_s_phoneme(&ps, self.british), rating));
            }
            return None;
        } else {
            return None;
        };

        self.lookup(&stem, tag).map(|(ps, rating)| (apply_s_phoneme(&ps, self.british), rating))
    }

    fn stem_ed(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        if word.len() < 4 || !word.ends_with('d') {
            return None;
        }

        let stem = if !word.ends_with("dd") && self.contains(&word[..word.len() - 1]) {
            word[..word.len() - 1].to_string()
        } else if word.len() > 4 && word.ends_with("ed") && !word.ends_with("eed") && self.contains(&word[..word.len() - 2]) {
            word[..word.len() - 2].to_string()
        } else {
            return None;
        };

        self.lookup(&stem, tag).map(|(ps, rating)| (apply_ed_phoneme(&ps, self.british), rating))
    }

    fn stem_ing(&self, word: &str, tag: Option<&str>) -> Option<(String, u8)> {
        if word.len() < 5 || !word.ends_with("ing") {
            return None;
        }

        let base = &word[..word.len() - 3];
        let stem = if word.len() > 5 && self.contains(base) {
            base.to_string()
        } else if self.contains(&format!("{}e", base)) {
            format!("{}e", base)
        } else if word.len() > 5 && is_doubled_consonant(word) && self.contains(&word[..word.len() - 4]) {
            word[..word.len() - 4].to_string()
        } else {
            return None;
        };

        self.lookup(&stem, tag).map(|(ps, rating)| (apply_ing_phoneme(&ps, self.british), rating))
    }
}

fn apply_s_phoneme(stem: &str, british: bool) -> String {
    if stem.is_empty() {
        return String::new();
    }

    let last = stem.chars().last().unwrap();
    if VOICELESS_ENDINGS.contains(last) && !SIBILANT_ENDINGS.contains(last) {
        format!("{}s", stem)
    } else if SIBILANT_ENDINGS.contains(last) {
        let schwa = if british { "ɪ" } else { "ᵻ" };
        format!("{}{}z", stem, schwa)
    } else {
        format!("{}z", stem)
    }
}

fn apply_ed_phoneme(stem: &str, british: bool) -> String {
    if stem.is_empty() {
        return String::new();
    }

    let last = stem.chars().last().unwrap();
    if "pkfθʃsʧ".contains(last) {
        format!("{}t", stem)
    } else if last == 'd' {
        let schwa = if british { "ɪ" } else { "ᵻ" };
        format!("{}{}d", stem, schwa)
    } else if last != 't' {
        format!("{}d", stem)
    } else if british || stem.len() < 2 {
        format!("{}ɪd", stem)
    } else {
        let chars: Vec<char> = stem.chars().collect();
        let second_last = chars[chars.len() - 2];
        if "AIOWYiuæɑəɛɪɹʊʌ".contains(second_last) {
            let base: String = chars[..chars.len() - 1].iter().collect();
            format!("{}ɾᵻd", base)
        } else {
            format!("{}ᵻd", stem)
        }
    }
}

fn apply_ing_phoneme(stem: &str, british: bool) -> String {
    if stem.is_empty() {
        return String::new();
    }

    if british {
        let last = stem.chars().last().unwrap();
        if "əː".contains(last) {
            return String::new();
        }
    } else if stem.len() > 1 {
        let chars: Vec<char> = stem.chars().collect();
        let last = chars[chars.len() - 1];
        let second_last = chars[chars.len() - 2];
        if last == 't' && "AIOWYiuæɑəɛɪɹʊʌ".contains(second_last) {
            let base: String = chars[..chars.len() - 1].iter().collect();
            return format!("{}ɾɪŋ", base);
        }
    }
    format!("{}ɪŋ", stem)
}

/// Apply stress modification to phonemes
pub fn apply_stress(phonemes: &str, stress: Option<i8>) -> String {
    let stress = match stress {
        Some(s) => s,
        None => return phonemes.to_string(),
    };

    if stress < -1 {
        // Remove all stress
        phonemes
            .chars()
            .filter(|&c| c != PRIMARY_STRESS && c != SECONDARY_STRESS)
            .collect()
    } else if stress == -1 || (stress == 0 && phonemes.contains(PRIMARY_STRESS)) {
        // Demote primary to secondary
        phonemes
            .replace(SECONDARY_STRESS_STR, "")
            .replace(PRIMARY_STRESS_STR, SECONDARY_STRESS_STR)
    } else if (stress == 0 || stress == 1)
        && !phonemes.contains(PRIMARY_STRESS)
        && !phonemes.contains(SECONDARY_STRESS)
    {
        // Add secondary stress
        if !phonemes.chars().any(|c| VOWELS.contains(c)) {
            phonemes.to_string()
        } else {
            format!("{}{}", SECONDARY_STRESS, phonemes)
        }
    } else if stress >= 1
        && !phonemes.contains(PRIMARY_STRESS)
        && phonemes.contains(SECONDARY_STRESS)
    {
        // Promote secondary to primary
        phonemes.replace(SECONDARY_STRESS_STR, PRIMARY_STRESS_STR)
    } else if stress > 1
        && !phonemes.contains(PRIMARY_STRESS)
        && !phonemes.contains(SECONDARY_STRESS)
    {
        // Add primary stress
        if !phonemes.chars().any(|c| VOWELS.contains(c)) {
            phonemes.to_string()
        } else {
            format!("{}{}", PRIMARY_STRESS, phonemes)
        }
    } else {
        phonemes.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexicon_lookup() {
        let lex = Lexicon::new(false);
        let result = lex.get_word("hello", None);
        assert!(result.is_some());
        let (phonemes, rating) = result.unwrap();
        assert!(!phonemes.is_empty());
        assert!(rating >= 3);
    }

    #[test]
    fn test_lexicon_ref() {
        let lex = Lexicon::new_static(false);
        let result = lex.get_word("world", None);
        assert!(result.is_some());
    }

    #[test]
    fn test_stemming_s() {
        let lex = Lexicon::new(false);
        let result = lex.get_word("cats", None);
        assert!(result.is_some());
    }

    #[test]
    fn test_stemming_ed() {
        let lex = Lexicon::new(false);
        let result = lex.get_word("walked", None);
        assert!(result.is_some());
    }

    #[test]
    fn test_stemming_ing() {
        let lex = Lexicon::new(false);
        let result = lex.get_word("walking", None);
        assert!(result.is_some());
    }

    #[test]
    fn test_apply_stress() {
        let ps = "hɛlO";
        assert_eq!(apply_stress(ps, Some(1)), "ˌhɛlO");
        assert_eq!(apply_stress(ps, Some(2)), "ˈhɛlO");
    }

    #[test]
    fn test_case_insensitive() {
        let lex = Lexicon::new(false);
        let lower = lex.get_word("hello", None);
        let upper = lex.get_word("HELLO", None);
        assert!(lower.is_some());
        assert!(upper.is_some());
    }
}
