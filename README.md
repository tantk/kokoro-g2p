# Kokoro G2P

A high-performance Rust library for Grapheme-to-Phoneme (G2P) conversion, designed for the Kokoro TTS model. Supports **10 languages** with optimized phoneme tokenization.

## Supported Languages

| Language | Code | Feature Flag | Accuracy |
|----------|------|--------------|----------|
| English (US) | `en-us` | `english` | 80% |
| English (UK) | `en-gb` | `english` | 80% |
| Spanish | `es` | `spanish` | 78% |
| Italian | `it` | `italian` | 49% |
| Indonesian | `id` | `indonesian` | 36% |
| Turkish | `tr` | `turkish` | 20% |
| Portuguese | `pt` | `portuguese` | 16% |
| German | `de` | `german` | 6% |
| Chinese | `zh` | `chinese` | - |
| Korean | `ko` | `korean` | - |
| Vietnamese | `vi` | `vietnamese` | - |

*Accuracy measured against WikiPron pronunciation dictionary. Lower scores for some languages are due to IPA notation differences, not pronunciation errors.*

## Features

- **Multi-language G2P** - 10 languages with unified API
- **Dictionary + Rules** - Hybrid approach for best accuracy
- **Text Normalization** - Numbers, currency, dates, abbreviations
- **Lazy Loading** - Language engines initialized on demand
- **Small Binary** - ~5MB per language, optimized for mobile
- **Multiple Bindings** - Rust, C FFI, JNI (Android), Python (via ctypes)

## Installation

### Rust

```toml
[dependencies]
kokoro-g2p = { version = "0.1", features = ["english"] }

# Or all languages
kokoro-g2p = { version = "0.1", features = ["full"] }
```

### Build from Source

```bash
# Clone
git clone https://github.com/hexgrad/misaki
cd misaki/native/kokoro-g2p

# Build with specific languages
cargo build --release --features "english spanish chinese"

# Build all languages
cargo build --release --features full
```

## Usage

### Rust

```rust
use kokoro_g2p::{text_to_tokens, text_to_phonemes, KPipeline};

// Simple API
let tokens = text_to_tokens("Hello, world!", "en-us");
let phonemes = text_to_phonemes("Hello, world!", "en-us");

// Pipeline API (recommended for multiple calls)
let mut pipeline = KPipeline::new("en-us");
let result = pipeline.process("Hello, world!");
println!("Phonemes: {}", result.phonemes);
println!("Tokens: {:?}", result.tokens);

// Switch languages
pipeline.set_language("es");
let spanish = pipeline.process("Hola, mundo!");
```

### Python (via ctypes)

```python
import ctypes
from ctypes import c_char_p, c_int64, POINTER, Structure

class CTokenArray(Structure):
    _fields_ = [("data", POINTER(c_int64)), ("len", ctypes.c_size_t)]

# Load library
lib = ctypes.CDLL("./target/release/kokoro_g2p.dll")  # or .so/.dylib

# Configure functions
lib.kokoro_text_to_tokens.argtypes = [c_char_p, c_char_p]
lib.kokoro_text_to_tokens.restype = CTokenArray
lib.kokoro_text_to_phonemes.argtypes = [c_char_p, c_char_p]
lib.kokoro_text_to_phonemes.restype = c_char_p
lib.kokoro_free_tokens.argtypes = [CTokenArray]
lib.kokoro_free_string.argtypes = [c_char_p]

# Use
result = lib.kokoro_text_to_tokens(b"Hello world", b"en-us")
tokens = [result.data[i] for i in range(result.len)]
lib.kokoro_free_tokens(result)

phonemes = lib.kokoro_text_to_phonemes(b"Hello world", b"en-us")
print(phonemes.decode())
lib.kokoro_free_string(phonemes)
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
val tokens = KokoroTokenizer.tokenizeWithLanguage("Hello!", "en-us")
val phonemes = KokoroTokenizer.textToPhonemes("Hola!", "es")
```

### C/C++

```c
#include <stdint.h>

typedef struct {
    int64_t* data;
    size_t len;
} CTokenArray;

// Functions
extern CTokenArray kokoro_text_to_tokens(const char* text, const char* language);
extern char* kokoro_text_to_phonemes(const char* text, const char* language);
extern void kokoro_free_tokens(CTokenArray array);
extern void kokoro_free_string(char* s);
extern const char* kokoro_version(void);

// Usage
CTokenArray tokens = kokoro_text_to_tokens("Hello", "en-us");
// Use tokens.data[0..tokens.len]
kokoro_free_tokens(tokens);

char* phonemes = kokoro_text_to_phonemes("Hello", "en-us");
printf("%s\n", phonemes);
kokoro_free_string(phonemes);
```

## Building for Mobile

### Android

```bash
# Install targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

# Install cargo-ndk
cargo install cargo-ndk

# Build
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o ./jniLibs build --release --features "jni full"
```

### iOS

```bash
# Install targets
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

# Build
cargo build --release --target aarch64-apple-ios --features full
cargo build --release --target aarch64-apple-ios-sim --features full

# Create XCFramework (recommended)
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/libkokoro_g2p.a \
    -library target/aarch64-apple-ios-sim/release/libkokoro_g2p.a \
    -output KokoroG2P.xcframework
```

## Validation & Testing

The library includes validation tests against [WikiPron](https://github.com/CUNY-CL/wikipron) pronunciation dictionaries.

```bash
# Run all tests
cargo test --features full

# Run validation tests with output
cargo test --features full --test validation -- --nocapture

# Test specific language
cargo test --features english --test validation test_validate_english_common -- --nocapture
```

## Language-Specific Notes

### English
- Dictionary-based with 100K+ entries
- Supports American (en-us) and British (en-gb) variants
- Text normalization: numbers, currency, time, dates

### Chinese
- Requires `chinese` feature (adds jieba-rs dependency)
- Pinyin to Zhuyin (Bopomofo) conversion
- Tone sandhi rules (3-3, 一, 不)
- Polyphone disambiguation

### Spanish/Italian
- Near-phonetic orthography
- High accuracy rule-based conversion

### Korean
- Hangul decomposition into jamo
- Phonological rules (liaison, nasalization)

### Vietnamese
- 6-tone detection from diacritics
- Northern (Hanoi) pronunciation

## Phoneme Vocabulary

178 tokens including:
- **Vowels**: A (eɪ), I (aɪ), O (oʊ), W (aʊ), ə, ɪ, ɛ, æ, ɑ, ɔ, ʊ, ʌ
- **Consonants**: b, d, f, g, k, l, m, n, p, s, t, v, z, θ, ð, ʃ, ʒ, ʧ, ʤ, ŋ, ɹ
- **Stress**: ˈ (primary), ˌ (secondary), ː (length)
- **Zhuyin**: ㄅㄆㄇㄈ... (Chinese)
- **Punctuation**: . , ! ? ; : — …

## Binary Size

| Configuration | Size |
|--------------|------|
| English only | ~14MB |
| Single language | ~5MB |
| Full (10 languages) | ~20MB |

## License

Apache-2.0

## Credits

Based on the [Misaki](https://github.com/hexgrad/misaki) G2P engine by hexgrad.
