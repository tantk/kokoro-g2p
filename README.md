# Kokoro G2P

A Rust implementation of Grapheme-to-Phoneme (G2P) conversion for the Kokoro TTS model.

## Overview

This library converts English text into phoneme token IDs that can be used with the Kokoro-82M TTS model. It supports:

- **American English** (en-us) - Default
- **British English** (en-gb)

## Features

- Dictionary-based pronunciation lookup (gold/silver dictionaries)
- Automatic text normalization:
  - Numbers to words ("123" → "one hundred twenty three")
  - Currency ("$50" → "fifty dollars")
  - Time ("2:30 PM" → "two thirty PM")
  - Ordinals ("1st" → "first")
  - Abbreviations ("Dr." → "Doctor")
- Stemming support for -s, -ed, -ing suffixes
- Contraction handling
- Acronym spelling
- JNI bindings for Android
- C FFI for iOS and other platforms

## Usage

### Rust

```rust
use kokoro_g2p::{text_to_tokens, text_to_phonemes};

// Convert text to token IDs
let tokens = text_to_tokens("Hello, world!", "en-us");
println!("Tokens: {:?}", tokens);

// Get phoneme representation
let phonemes = text_to_phonemes("Hello, world!", "en-us");
println!("Phonemes: {}", phonemes);
```

### Android (Kotlin)

```kotlin
object KokoroTokenizer {
    init {
        System.loadLibrary("kokoro_g2p")
    }

    external fun tokenize(text: String): LongArray
    external fun tokenizeWithLanguage(text: String, language: String): LongArray
    external fun textToPhonemes(text: String, language: String): String
}

// Usage
val tokens = KokoroTokenizer.tokenize("Hello, world!")
```

### iOS (Swift)

```swift
import KokoroG2P

// Convert text to tokens
let tokens = kokoro_text_to_tokens("Hello, world!", "en-us")
defer { kokoro_free_tokens(tokens) }

let tokenArray = Array(UnsafeBufferPointer(start: tokens.data, count: tokens.len))
```

## Building

### For Development

```bash
cargo build
cargo test
```

### For Android

```bash
# Install targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

# Install cargo-ndk
cargo install cargo-ndk

# Build
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o ./jniLibs build --release --features jni
```

### For iOS

```bash
# Install targets
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

# Build
cargo build --release --target aarch64-apple-ios
cargo build --release --target x86_64-apple-ios
cargo build --release --target aarch64-apple-ios-sim

# Create universal binary
lipo -create \
    target/aarch64-apple-ios/release/libkokoro_g2p.a \
    target/x86_64-apple-ios/release/libkokoro_g2p.a \
    -output libkokoro_g2p_universal.a
```

## Phoneme Vocabulary

The library uses the Kokoro phoneme vocabulary with 178 tokens. Key symbols include:

- **Vowels**: A (eɪ), I (aɪ), O (oʊ), W (aʊ), Y (ɔɪ), ə, ɪ, ɛ, æ, ɑ, ɔ, ʊ, ʌ, etc.
- **Consonants**: b, d, f, g, h, k, l, m, n, p, s, t, v, w, z, θ, ð, ʃ, ʒ, ʧ, ʤ, ŋ, ɹ, etc.
- **Stress markers**: ˈ (primary), ˌ (secondary)
- **Punctuation**: . , ! ? ; : — …

## License

Apache-2.0 (matching the original Misaki project)

## Credits

Based on the [Misaki](https://github.com/hexgrad/misaki) G2P engine by hexgrad.
