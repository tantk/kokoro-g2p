# Kokoro G2P - Development Guide

## Project Overview

Rust G2P (Grapheme-to-Phoneme) library for Kokoro TTS model. Converts text to phoneme token IDs across multiple languages.

## Useful Rust Crates

### G2P & Phoneme Libraries
- [grapheme_to_phoneme](https://lib.rs/crates/grapheme_to_phoneme) - OOV prediction model (port of g2p.py)
- [espeakng](https://docs.rs/espeakng) - Safe wrapper for espeak-ng phonemization
- [espeakng-sys](https://github.com/Better-Player/espeakng-sys) - Low-level FFI bindings
- [espeak-rs](https://github.com/SpeechifyInc/espeak-rs) - Fast phonemization (~1ms/50chars)

### Tokenizer Crates (for reference)
- [tokenizers](https://crates.io/crates/tokenizers) - Hugging Face tokenizers (BPE, WordPiece)
- [rust_tokenizers](https://docs.rs/rust_tokenizers) - BERT/GPT2/XLNet tokenizers
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) - OpenAI tiktoken

### Static Maps & Performance
- [phf](https://lib.rs/crates/phf) - Compile-time perfect hash maps (we use this)
- [once_cell](https://docs.rs/once_cell) - Lazy static initialization
- [regex](https://docs.rs/regex) - Text pattern matching

### Language-Specific
- [jieba-rs](https://crates.io/crates/jieba-rs) - Chinese word segmentation (used for zh)
- [pinyin](https://crates.io/crates/pinyin) - Pinyin conversion fallback (used for zh)
- [mecab](https://crates.io/crates/mecab) - Japanese morphological analysis (reference, not currently used)
- [lindera](https://crates.io/crates/lindera) - Japanese/Korean/Chinese tokenizer (reference, not currently used)

## Architecture

```
src/
├── lib.rs           # Entry point, FFI/JNI exports, language dispatch
├── pipeline.rs      # KPipeline unified multi-language interface
├── tokenizer.rs     # Phoneme ↔ Token ID mapping (IPA vocabulary)
├── g2p.rs           # English G2P engine
├── lexicon.rs       # Dictionary lookup with stemming
├── preprocessor.rs  # English text normalization
├── zh/              # Chinese module (jieba segmentation + pinyin)
│   ├── mod.rs
│   ├── normalizer.rs
│   ├── phoneme_mapper.rs  # Pinyin to IPA/Zhuyin mapping
│   ├── pinyin.rs          # Hanzi to Pinyin conversion
│   ├── tone_sandhi.rs     # Tone sandhi rules (3-3, 一, 不)
│   ├── polyphone.rs       # Polyphonic character resolution
│   └── segmenter.rs       # Jieba word segmentation wrapper
├── ja/              # Japanese module (kanji reading + kana to IPA)
│   ├── mod.rs
│   ├── phoneme_map.rs     # Kana to phoneme mapping
│   └── reading.rs         # Kanji reading dictionary
├── es/              # Spanish module (rule-based)
├── de/              # German module (rule-based)
├── pt/              # Portuguese module (rule-based)
├── ko/              # Korean module (Hangul-based)
├── vi/              # Vietnamese module (rule-based)
├── id/              # Indonesian module (rule-based)
├── tr/              # Turkish module (rule-based)
└── it/              # Italian module (rule-based)
```

## Feature Flags Pattern

```toml
[features]
default = ["english"]
english = []
chinese = ["english", "dep:jieba-rs", "dep:pinyin"]  # Bilingual with English
japanese = ["english"]                                 # With English support
spanish = []
indonesian = []
turkish = []
italian = []
german = []
portuguese = []
korean = []
vietnamese = []
full = ["english", "chinese", "japanese", "spanish", "indonesian", "turkish", "italian", "german", "portuguese", "korean", "vietnamese"]
jni = ["dep:jni"]       # Android JNI interface
uniffi = ["dep:uniffi"]  # Alternative FFI
```

Use `#[cfg(feature = "langname")]` for conditional compilation:
```rust
#[cfg(feature = "spanish")]
pub mod es;

#[cfg(feature = "spanish")]
"es" | "spanish" => es::text_to_tokens(text),
```

## Adding a New Language

### 1. Create module structure
```
src/{lang_code}/
├── mod.rs           # {Lang}G2P struct, text_to_phonemes(), text_to_tokens()
└── normalizer.rs    # number_to_{lang}(), currency normalization
```

### 2. Implement G2P struct
```rust
pub struct {Lang}G2P;

impl {Lang}G2P {
    pub fn new() -> Self { Self }

    pub fn text_to_phonemes(&self, text: &str) -> String {
        let normalized = normalizer::normalize(text);
        // Convert words to IPA phonemes
    }

    pub fn text_to_tokens(&self, text: &str) -> Vec<i64> {
        let phonemes = self.text_to_phonemes(text);
        crate::tokenizer::phonemes_to_tokens(&phonemes)
    }
}

// Convenience functions
pub fn text_to_tokens(text: &str) -> Vec<i64> { ... }
pub fn text_to_phonemes(text: &str) -> String { ... }
```

### 3. Update Cargo.toml
```toml
langname = []
full = ["english", "chinese", ..., "langname"]
```

### 4. Update lib.rs
```rust
#[cfg(feature = "langname")]
pub mod ln;

// In text_to_tokens():
#[cfg(feature = "langname")]
"ln" | "langname" => ln::text_to_tokens(text),
```

### 5. Update pipeline.rs
- Add to `Language` enum
- Add `{lang}_g2p: Option<{Lang}G2P>` to KPipeline
- Add match arm in `process()`

## G2P Implementation Patterns

### Rule-based (Spanish, Indonesian, Turkish, Italian, German, Portuguese, Korean, Vietnamese)
```rust
fn word_to_phonemes(word: &str) -> String {
    let chars: Vec<char> = word.to_lowercase().chars().collect();
    let mut phonemes = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();

        // Check digraphs first
        match (c, next) {
            ('c', Some('h')) => { phonemes.push('ʧ'); i += 2; continue; }
            ('n', Some('g')) => { phonemes.push('ŋ'); i += 2; continue; }
            _ => {}
        }

        // Single character mapping
        match c {
            'a' => phonemes.push('a'),
            'b' => phonemes.push('b'),
            // ...
            _ => {}
        }
        i += 1;
    }
    phonemes
}
```

### Dictionary-based (English)
```rust
fn word_to_phonemes(word: &str) -> String {
    // 1. Dictionary lookup (gold → silver → stemming)
    if let Some(phonemes) = lexicon.lookup(word) {
        return phonemes;
    }
    // 2. Fallback rules
    fallback_g2p(word)
}
```

### Segmentation-based (Chinese, Japanese, Thai)
```rust
fn text_to_phonemes(&self, text: &str) -> String {
    let normalized = normalizer::normalize(text);
    let segments = self.segmenter.segment(&normalized);  // jieba-rs, mecab, etc.

    segments.iter()
        .map(|seg| convert_segment(seg))
        .collect()
}
```

## Normalizer Pattern

```rust
use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());
static CURRENCY_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"€\s*(\d+)").unwrap());

pub fn normalize(text: &str) -> String {
    let mut result = text.to_string();
    result = normalize_currency(&result);
    result = normalize_numbers(&result);
    result
}

pub fn number_to_{lang}(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        // ... language-specific number words
        10..=19 => format!("{} {}", base, number_to_{lang}(n - 10)),
        // ...
    }
}
```

## Tokenizer Vocabulary

IPA phonemes mapped to token IDs (0-255 range). Key ranges:
- 0: PAD token
- 1-17: Punctuation
- 18-42: Special symbols & diphthongs (including Japanese clusters)
- 43-68: Lowercase letters (a-z)
- 69-160: IPA vowels and consonants
- 156-158: Stress markers (ˈ, ˌ, ː)
- 169-173: Intonation markers (↓, →, ↗, ↘)
- 177: Reduced vowel (ᵻ)
- 180-216: Zhuyin/Bopomofo (Chinese)
- 217+: Additional phonemes (e.g., Turkish ʏ)

Add new phonemes in tokenizer.rs VOCAB map:
```rust
m.insert('ʏ', 217);  // Near-close near-front rounded vowel
```

## Testing Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let g2p = {Lang}G2P::new();
        let phonemes = g2p.text_to_phonemes("hello");
        assert!(!phonemes.is_empty());
    }

    #[test]
    fn test_digraph() {
        let phonemes = word_to_phonemes("word_with_digraph");
        assert!(phonemes.contains('ʧ'));  // Expected IPA symbol
    }

    #[test]
    fn test_tokens_not_empty() {
        let tokens = text_to_tokens("test phrase", "ln");
        assert!(tokens.len() > 2);  // More than just padding
        assert_eq!(tokens[0], 0);   // Start padding
        assert_eq!(*tokens.last().unwrap(), 0);  // End padding
    }
}
```

## Build Commands

```bash
# Development
cargo build --features spanish
cargo test --features spanish

# Single language
cargo build --release --features spanish

# All languages
cargo build --release --features full
cargo test --features full

# Check binary size
ls -lh target/release/libkokoro_g2p.so
```

## Implemented Languages (11)

### Complex with Dependencies
- **English** - Dictionary-based (gold → silver → stemming → fallback rules), ~14K lines
- **Chinese** - Jieba segmentation + pinyin + tone sandhi + Zhuyin mapping, ~2.5K lines, includes embedded English G2P for bilingual text
- **Japanese** - Kanji reading dictionary + kana to IPA, ~1.1K lines

### Rule-based (~200-400 lines each)
- **Spanish**, **German**, **Portuguese**, **Korean**, **Vietnamese**, **Indonesian**, **Turkish**, **Italian**
- Near-phonetic orthography, no external dependencies

## Not Yet Implemented (for reference)

### Hard (ML/Complex processing)
- Hindi (schwa deletion), Russian (stress), French (liaison)
- Need large dictionaries or statistical models

### Very Hard (Preprocessing required)
- Arabic (diacritization), Thai (segmentation)
- Need separate ML models before G2P

## Common IPA Symbols

| Symbol | Sound | Example | ID |
|--------|-------|---------|-----|
| ʧ | ch | church | 133 |
| ʤ | j | judge | 82 |
| ʃ | sh | ship | 131 |
| ʒ | zh | vision | 147 |
| ŋ | ng | sing | 112 |
| ɲ | ny | canyon | 114 |
| ʎ | ly | Italian gli | 143 |
| ɾ | tap | butter | 125 |
| ʔ | glottal | uh-oh | 148 |
| ə | schwa | about | 83 |
| ˈ | stress | ˈhello | 156 |
| ː | long | beːt | 158 |

## PHF Best Practices

Using `phf` for compile-time perfect hash maps (zero runtime collision):

```toml
[dependencies]
phf = { version = "0.11", features = ["macros"] }
```

```rust
use phf::phf_map;

// Compile-time static map - O(1) lookup, no collisions
static WORD_TO_PHONEME: phf::Map<&'static str, &'static str> = phf_map! {
    "hello" => "hˈɛloʊ",
    "world" => "wˈɜɹld",
};

// Usage
if let Some(phoneme) = WORD_TO_PHONEME.get(word) {
    return phoneme.to_string();
}
```

For dynamic maps, use `once_cell::sync::Lazy<HashMap>`:
```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;

static VOCAB: Lazy<HashMap<char, i64>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('a', 43);
    m.insert('b', 44);
    m
});
```

## espeak-ng Fallback (Optional)

For languages without rule-based G2P, use espeak-ng as fallback:

```toml
[dependencies]
espeakng = { version = "0.4", optional = true }

[features]
espeak-fallback = ["dep:espeakng"]
```

```rust
#[cfg(feature = "espeak-fallback")]
fn espeak_phonemize(text: &str, lang: &str) -> String {
    use espeakng::{initialise, Speaker};

    initialise(None).expect("espeak init");
    let speaker = Speaker::new();
    speaker.text_to_phonemes(text, lang).unwrap_or_default()
}
```

## Performance Tips

1. **Lazy initialization** - Load dictionaries on first use, not at startup
2. **PHF for static lookups** - Use `phf_map!` for word→phoneme dictionaries
3. **Avoid allocations** - Use `&str` references where possible
4. **Batch processing** - Process multiple sentences together
5. **Feature flags** - Only compile needed languages to reduce binary size

## Binary Size Optimization

```toml
[profile.release]
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
opt-level = "z"      # Size optimization
panic = "abort"      # Smaller panic handling
strip = true         # Strip symbols
```

Expected sizes:
- English only: ~14MB
- Single language: ~5MB
- Full (all languages): ~20MB
