//! Phoneme to Token ID mapping for Kokoro TTS model
//!
//! This module provides the vocabulary mapping from phonemes to token IDs
//! as expected by the Kokoro-82M model.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Maximum number of tokens (excluding padding)
pub const MAX_TOKENS: usize = 510;

/// Padding token ID (used at start and end)
pub const PAD_TOKEN: i64 = 0;

/// The Kokoro vocabulary mapping phonemes/symbols to token IDs
static VOCAB: Lazy<HashMap<char, i64>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // Punctuation
    m.insert(';', 1);
    m.insert(':', 2);
    m.insert(',', 3);
    m.insert('.', 4);
    m.insert('!', 5);
    m.insert('?', 6);
    m.insert('—', 9);    // em-dash
    m.insert('…', 10);   // ellipsis
    m.insert('"', 11);   // straight quote
    m.insert('(', 12);
    m.insert(')', 13);
    m.insert('"', 14);   // left double quote
    m.insert('"', 15);   // right double quote
    m.insert(' ', 16);   // space
    m.insert('\u{0303}', 17); // combining tilde

    // Consonant clusters (Japanese/other)
    m.insert('ʣ', 18);
    m.insert('ʥ', 19);
    m.insert('ʦ', 20);
    m.insert('ʨ', 21);
    m.insert('ᵝ', 22);
    m.insert('\u{AB67}', 23); // modifier letter small theta

    // Diphthong vowels (capital letters)
    m.insert('A', 24);   // "eh" sound (eɪ)
    m.insert('I', 25);   // "eye" sound (aɪ)
    m.insert('O', 31);   // American "oh" (oʊ)
    m.insert('Q', 33);   // British "oh" (əʊ)
    m.insert('S', 35);   // Special consonant cluster
    m.insert('T', 36);   // Flap T (American ɾ variant)
    m.insert('W', 39);   // "ow" sound (aʊ)
    m.insert('Y', 41);   // "oy" sound (ɔɪ)

    // Small schwa
    m.insert('ᵊ', 42);

    // Lowercase letters (used for consonants)
    m.insert('a', 43);   // British ash vowel
    m.insert('b', 44);
    m.insert('c', 45);
    m.insert('d', 46);
    m.insert('e', 47);
    m.insert('f', 48);
    m.insert('h', 50);
    m.insert('i', 51);   // as in "easy"
    m.insert('j', 52);   // "y" sound in IPA
    m.insert('k', 53);
    m.insert('l', 54);
    m.insert('m', 55);
    m.insert('n', 56);
    m.insert('o', 57);
    m.insert('p', 58);
    m.insert('q', 59);
    m.insert('r', 60);
    m.insert('s', 61);
    m.insert('t', 62);
    m.insert('u', 63);   // as in "flu"
    m.insert('v', 64);
    m.insert('w', 65);
    m.insert('x', 66);
    m.insert('y', 67);
    m.insert('z', 68);

    // IPA vowels
    m.insert('ɑ', 69);   // "spa"
    m.insert('ɐ', 70);   // near-open central vowel
    m.insert('ɒ', 71);   // British "on"
    m.insert('æ', 72);   // American "ash"
    m.insert('β', 75);   // bilabial fricative
    m.insert('ɔ', 76);   // "all"
    m.insert('ɕ', 77);   // voiceless alveolo-palatal fricative
    m.insert('ç', 78);   // voiceless palatal fricative
    m.insert('ɖ', 80);   // voiced retroflex plosive
    m.insert('ð', 81);   // "than" (soft th)
    m.insert('ʤ', 82);   // "jump" (dʒ)
    m.insert('ə', 83);   // schwa
    m.insert('ɚ', 85);   // r-colored schwa
    m.insert('ɛ', 86);   // "bed"
    m.insert('ɜ', 87);   // "her"
    m.insert('ɟ', 90);   // voiced palatal plosive
    m.insert('ɡ', 92);   // "get" (hard g)
    m.insert('ɥ', 99);   // labial-palatal approximant
    m.insert('ɨ', 101);  // close central unrounded vowel
    m.insert('ɪ', 102);  // "brick"
    m.insert('ʝ', 103);  // voiced palatal fricative
    m.insert('ɯ', 110);  // close back unrounded vowel
    m.insert('ɰ', 111);  // velar approximant
    m.insert('ŋ', 112);  // "sung"
    m.insert('ɳ', 113);  // retroflex nasal
    m.insert('ɲ', 114);  // palatal nasal
    m.insert('ɴ', 115);  // uvular nasal
    m.insert('ø', 116);  // close-mid front rounded vowel
    m.insert('ɸ', 118);  // voiceless bilabial fricative
    m.insert('θ', 119);  // "thin" (hard th)
    m.insert('œ', 120);  // open-mid front rounded vowel
    m.insert('ɹ', 123);  // "red" (upside-down r)
    m.insert('ɾ', 125);  // American flap "butter"
    m.insert('ɻ', 126);  // retroflex approximant
    m.insert('ʁ', 128);  // voiced uvular fricative
    m.insert('ɽ', 129);  // retroflex flap
    m.insert('ʂ', 130);  // voiceless retroflex fricative
    m.insert('ʃ', 131);  // "shin"
    m.insert('ʈ', 132);  // voiceless retroflex plosive
    m.insert('ʧ', 133);  // "chump" (tʃ)
    m.insert('ʊ', 135);  // "wood"
    m.insert('ʋ', 136);  // labiodental approximant
    m.insert('ʌ', 138);  // "sun"
    m.insert('ɣ', 139);  // voiced velar fricative
    m.insert('ɤ', 140);  // close-mid back unrounded vowel
    m.insert('χ', 142);  // voiceless uvular fricative
    m.insert('ʎ', 143);  // palatal lateral approximant
    m.insert('ʒ', 147);  // "vision" (zh)
    m.insert('ʔ', 148);  // glottal stop

    // Stress markers
    m.insert('ˈ', 156);  // primary stress
    m.insert('ˌ', 157);  // secondary stress
    m.insert('ː', 158);  // vowel lengthener

    // Aspiration and palatalization
    m.insert('ʰ', 162);  // aspiration
    m.insert('ʲ', 164);  // palatalization

    // Intonation markers
    m.insert('↓', 169);  // falling pitch
    m.insert('→', 171);  // level pitch
    m.insert('↗', 172);  // rising pitch
    m.insert('↘', 173);  // falling pitch

    // American reduced vowel
    m.insert('ᵻ', 177);  // between ə and ɪ

    m
});

/// Reverse mapping from token ID to character
static ID_TO_CHAR: Lazy<HashMap<i64, char>> = Lazy::new(|| {
    VOCAB.iter().map(|(&c, &id)| (id, c)).collect()
});

/// Convert a single phoneme character to its token ID
#[inline]
pub fn phoneme_to_id(phoneme: char) -> Option<i64> {
    VOCAB.get(&phoneme).copied()
}

/// Convert a token ID back to its phoneme character
#[inline]
pub fn id_to_phoneme(id: i64) -> Option<char> {
    ID_TO_CHAR.get(&id).copied()
}

/// Convert a phoneme string to a vector of token IDs
/// Returns tokens padded with 0 at start and end
pub fn phonemes_to_tokens(phonemes: &str) -> Vec<i64> {
    let mut tokens = Vec::with_capacity(phonemes.len() + 2);
    tokens.push(PAD_TOKEN); // Start padding

    for c in phonemes.chars() {
        if let Some(id) = phoneme_to_id(c) {
            tokens.push(id);
        } else {
            // Log unknown character but don't crash
            log::warn!("Unknown phoneme character: {:?} (U+{:04X})", c, c as u32);
        }
    }

    tokens.push(PAD_TOKEN); // End padding

    // Truncate if exceeding max length
    if tokens.len() > MAX_TOKENS + 2 {
        tokens.truncate(MAX_TOKENS + 1);
        tokens.push(PAD_TOKEN);
    }

    tokens
}

/// Convert token IDs back to phoneme string
pub fn tokens_to_phonemes(tokens: &[i64]) -> String {
    tokens
        .iter()
        .filter(|&&id| id != PAD_TOKEN)
        .filter_map(|&id| id_to_phoneme(id))
        .collect()
}

/// Check if a character is a valid phoneme in the vocabulary
#[inline]
pub fn is_valid_phoneme(c: char) -> bool {
    VOCAB.contains_key(&c)
}

/// Get all valid phoneme characters
pub fn get_vocabulary() -> Vec<char> {
    let mut chars: Vec<char> = VOCAB.keys().copied().collect();
    chars.sort_by_key(|c| VOCAB.get(c).unwrap_or(&0));
    chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phoneme_to_id() {
        assert_eq!(phoneme_to_id(' '), Some(16));
        assert_eq!(phoneme_to_id('.'), Some(4));
        assert_eq!(phoneme_to_id('ˈ'), Some(156));
        assert_eq!(phoneme_to_id('ə'), Some(83));
        assert_eq!(phoneme_to_id('🚀'), None);
    }

    #[test]
    fn test_phonemes_to_tokens() {
        let tokens = phonemes_to_tokens("hˈɛlO");
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
        assert!(tokens.len() > 2);
    }

    #[test]
    fn test_roundtrip() {
        let phonemes = "hˈɛlO wˈɜɹld";
        let tokens = phonemes_to_tokens(phonemes);
        let recovered = tokens_to_phonemes(&tokens);
        assert_eq!(recovered, phonemes);
    }

    #[test]
    fn test_max_length() {
        let long_phonemes = "ə".repeat(600);
        let tokens = phonemes_to_tokens(&long_phonemes);
        assert!(tokens.len() <= MAX_TOKENS + 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    fn test_vocabulary_size() {
        // Kokoro has 178 tokens total (including PAD at 0)
        let vocab = get_vocabulary();
        assert!(vocab.len() > 100); // Should have most of them
    }
}
