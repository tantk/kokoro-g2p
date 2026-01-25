# Multi-Language G2P Implementation Handover

## Overview

Successfully implemented multi-language G2P support for the kokoro-g2p Rust library. The implementation extends the existing English G2P engine while maintaining backward compatibility.

**Supported Languages:**
- English (en-us, en-gb) - Original
- Mandarin Chinese (zh-CN) - Phase 0
- Spanish (es) - Phase 1
- Indonesian (id) - Phase 1
- Turkish (tr) - Phase 1
- Italian (it) - Phase 1
- **German (de)** - Phase 2 NEW
- **Portuguese (pt)** - Phase 2 NEW
- **Korean (ko)** - Phase 2 NEW
- **Vietnamese (vi)** - Phase 2 NEW

## Completed Work

### 1. Module Structure Created

```
src/
├── lib.rs                    # Updated with language dispatch
├── pipeline.rs               # NEW: KPipeline unified interface
├── tokenizer.rs              # Extended with Zhuyin tokens
├── zh/                       # NEW: Chinese module
│   ├── mod.rs                # ChineseG2P struct and public API
│   ├── segmenter.rs          # jieba-rs wrapper with POS tagging
│   ├── pinyin.rs             # Hanzi → Pinyin conversion
│   ├── tone_sandhi.rs        # Tone change rules (3-3, 一, 不)
│   ├── polyphone.rs          # Polyphone dictionary (~100 entries)
│   ├── normalizer.rs         # Text normalization
│   └── phoneme_mapper.rs     # Pinyin → Zhuyin mapping
```

### 2. Cargo.toml Configuration

See Phase 2 section for full feature configuration.

```toml
[dependencies]
jieba-rs = { version = "0.7", optional = true }
uniffi = { version = "0.28", optional = true }
```

### 3. Key Implementations

#### Text Normalization (`normalizer.rs`)
- Numbers: 123 → 一百二十三
- Currency: S$50 → 新加坡元五十, ¥100 → 一百元, $50 → 美元五十
- Dates: 2024年1月15日 → 二零二四年一月十五日
- Percentages: 50% → 百分之五十
- Phone numbers: digit-by-digit reading

#### Tone Sandhi (`tone_sandhi.rs`)
- Third tone sandhi: 3-3 → 2-3 (你好 → ní hǎo)
- 一 rules: yi1+4th→yi2, yi1+other→yi4
- 不 rules: bu4+4th→bu2

#### Polyphone Resolution (`polyphone.rs`)
- Phrase-based lookup (highest priority): 银行→yín háng, 行走→xíng zǒu
- POS-based disambiguation: 行 as verb→xíng, as noun→háng
- Default fallback for common characters

#### Phoneme Mapping (`phoneme_mapper.rs`)
- Full Pinyin → Zhuyin (Bopomofo) conversion
- Tone markers: 1→'→', 2→'↗', 3→'↓', 4→'↘', 5→(none)
- Handles all standard Mandarin syllables

#### Tokenizer Extension (`tokenizer.rs`)
Added Zhuyin characters (IDs 180-216):
- Initials: ㄅㄆㄇㄈㄉㄊㄋㄌㄍㄎㄏㄐㄑㄒㄓㄔㄕㄖㄗㄘㄙ
- Finals: ㄧㄨㄩㄚㄛㄜㄝㄞㄟㄠㄡㄢㄣㄤㄥㄦ

### 4. API

```rust
// Simple functions
let tokens = text_to_tokens("你好世界", "zh");
let phonemes = text_to_phonemes("你好世界", "zh");

// Pipeline API
let mut pipeline = KPipeline::new("zh");
let result = pipeline.process("你好世界");
// result.phonemes: "ㄋㄧ↗ ㄏㄠ↓ ㄕ↘ ㄐㄧㄝ↘"
// result.tokens: [0, 186, 201, 172, 16, 190, 210, 169, ...]

// Language codes supported: "zh", "zh-cn", "chinese", "mandarin", "cmn"
```

### 5. Test Coverage (Phase 0)

**Chinese-specific tests** (see Phase 2 for full test count):
- Tone sandhi tests (你好, 一个, 不是)
- Polyphone tests (行走 vs 银行)
- Currency normalization (S$, ¥, $)
- Number conversion (0-99999+)
- Pipeline integration tests

## Binary Sizes

| Configuration | Size | Notes |
|--------------|------|-------|
| English only | 14MB | Default build |
| Chinese only | 4.6MB | Minimal Chinese |
| Full | 17MB | Both languages |

**Note**: Full build is 17MB, slightly over 15MB target due to jieba-rs embedded dictionary.

---

## Phase 1 Languages ✅ COMPLETED (Spanish, Indonesian, Turkish, Italian)

### Module Structure

```
src/
├── es/                       # Spanish module
│   ├── mod.rs                # SpanishG2P struct and public API
│   └── normalizer.rs         # Number/currency to Spanish words
├── id/                       # Indonesian module
│   ├── mod.rs                # IndonesianG2P struct and public API
│   └── normalizer.rs         # Number/currency to Indonesian words
├── tr/                       # Turkish module
│   ├── mod.rs                # TurkishG2P struct and public API
│   └── normalizer.rs         # Number/currency to Turkish words
└── it/                       # Italian module
    ├── mod.rs                # ItalianG2P struct and public API
    └── normalizer.rs         # Number/currency to Italian words
```

### Cargo.toml Configuration

```toml
[features]
default = ["english"]
english = []
chinese = ["dep:jieba-rs"]
spanish = []
indonesian = []
turkish = []
italian = []
full = ["english", "chinese", "spanish", "indonesian", "turkish", "italian"]
```

### Key Features Per Language

#### Spanish
- Near-phonetic orthography (very regular rules)
- Digraphs: ch→/ʧ/, ll→/ʝ/, rr→/r/, qu→/k/, gu→/ɡ/
- Latin American pronunciation (s/z→/s/ instead of Castilian /θ/)
- Stress rules: penultimate default, accent marks override
- Number normalization: 0-999,999,999
- Currency: €, $

#### Indonesian
- Very transparent orthography
- Digraphs: ng→/ŋ/, ny→/ɲ/, sy→/ʃ/, kh→/x/, gh→/ɣ/
- c→/ʧ/ (always), j→/ʤ/
- Final k→/ʔ/ (glottal stop)
- Schwa handling for letter 'e'
- Currency: Rp (Rupiah), $

#### Turkish
- Near-phonetic with special characters
- Vowel harmony (8 vowels): a, e, ı, i, o, ö, u, ü
- Special consonants: ç→/ʧ/, ş→/ʃ/, ğ→lengthening/j
- c→/ʤ/, j→/ʒ/
- Currency: ₺ (Lira), TL, €, $

#### Italian
- Regular orthography with gemination
- Trigraphs: gli→/ʎ/, sci→/ʃ/
- Digraphs: gn→/ɲ/, ch→/k/, gh→/ɡ/
- Double consonants marked with length /ː/
- Open/closed vowels: è→/ɛ/, é→/e/, ò→/ɔ/, ó→/o/
- z→/ʦ/
- Currency: €, $

### API Usage

```rust
// Simple functions
let tokens = text_to_tokens("hola mundo", "es");
let phonemes = text_to_phonemes("selamat pagi", "id");
let tokens = text_to_tokens("merhaba dünya", "tr");
let phonemes = text_to_phonemes("ciao mondo", "it");

// Pipeline API
let mut pipeline = KPipeline::new("es");
let result = pipeline.process("Buenos días");
// result.phonemes: "bˈuenosˈdias"
// result.tokens: [0, 44, 156, 63, 47, 56, 57, 61, ...]

// Language codes supported:
// Spanish: "es", "es-es", "es-mx", "spanish", "español"
// Indonesian: "id", "indonesian", "bahasa"
// Turkish: "tr", "turkish", "türkçe"
// Italian: "it", "italian", "italiano"
```

### Test Results (Phase 1)

Phase 1 language tests (included in total 173 tests):
- Spanish: 9 tests (G2P, digraphs, stress, normalization)
- Indonesian: 8 tests (G2P, digraphs, glottal stop, normalization)
- Turkish: 10 tests (G2P, vowels, consonants, soft g, normalization)
- Italian: 10 tests (G2P, digraphs, trigraphs, gemination, normalization)

---

## Phase 2 Languages ✅ COMPLETED (German, Portuguese, Korean, Vietnamese)

### Module Structure

```
src/
├── de/                       # German module
│   ├── mod.rs                # GermanG2P struct and public API
│   └── normalizer.rs         # Number/currency to German words
├── pt/                       # Portuguese module
│   ├── mod.rs                # PortugueseG2P struct and public API
│   └── normalizer.rs         # Number/currency to Portuguese words
├── ko/                       # Korean module
│   ├── mod.rs                # KoreanG2P struct with Hangul decomposition
│   └── normalizer.rs         # Sino-Korean number words
└── vi/                       # Vietnamese module
    ├── mod.rs                # VietnameseG2P struct with tone detection
    └── normalizer.rs         # Vietnamese number words
```

### Cargo.toml Configuration

```toml
[features]
default = ["english"]
english = []
chinese = ["dep:jieba-rs"]
spanish = []
indonesian = []
turkish = []
italian = []
german = []
portuguese = []
korean = []
vietnamese = []
full = ["english", "chinese", "spanish", "indonesian", "turkish", "italian", "german", "portuguese", "korean", "vietnamese"]
```

### Key Features Per Language

#### German
- Umlauts: ä→/ɛ/, ö→/ø/, ü→/y/
- ß (Eszett)→/s/
- Digraphs: sch→/ʃ/, ch→context-dependent (ich-Laut /ç/ vs ach-Laut /x/)
- ie→/iː/, ei→/aɪ/, eu/äu→/ɔʏ/, au→/aʊ/
- Final devoicing: b→/p/, d→/t/, g→/k/ at word end
- Number system: inverted ones-tens (einundzwanzig = "one-and-twenty")
- Currency: €, $

#### Portuguese
- Brazilian Portuguese as default (brazilian: bool flag available)
- Nasal vowels: ã→/ɐ̃/, õ→/õ/
- Digraphs: lh→/ʎ/, nh→/ɲ/, ch→/ʃ/
- Brazilian palatalization: ti→/ʧi/, di→/ʤi/
- Open vowels: é→/ɛ/, ó→/ɔ/
- Currency: R$ (Real), €, $

#### Korean
- Full Hangul decomposition into jamo (choseong, jungseong, jongseong)
- Unicode-based syllable block processing (0xAC00-0xD7A3 range)
- Initial consonants (19): ㄱ→/k/, ㄴ→/n/, etc.
- Medial vowels (21): ㅏ→/a/, ㅓ→/ʌ/, etc.
- Final consonants (27): ㄱ→/k̚/, ㄴ→/n/, etc.
- Phonological rules: liaison (연음), nasalization (비음화)
- Sino-Korean number system: 일, 이, 삼... 십, 백, 천, 만, 억
- Currency: ₩ (Won), $

#### Vietnamese
- 6 tone system detected from diacritics:
  - Ngang (level): no mark → /→/
  - Huyền (falling): à, ằ, ầ... → /↘/
  - Sắc (rising): á, ắ, ấ... → /↗/
  - Hỏi (dipping): ả, ẳ, ẩ... → /↓/
  - Ngã (rising glottalized): ã, ẵ, ẫ... → /↗/
  - Nặng (low falling): ạ, ặ, ậ... → /↘/
- Trigraphs: ngh→/ŋ/
- Digraphs: ng→/ŋ/, nh→/ɲ/, ch→/c/, th→/tʰ/, tr→/ʈ/, ph→/f/, kh→/x/, gh→/ɣ/, gi→/z/, qu→/kw/
- Vowel modifications: ă, â→/ə/, ê→/e/, ô→/o/, ơ→/ɤ/, ư→/ɯ/
- Northern Vietnamese pronunciation (d→/z/, đ→/d/)
- Currency: đ, ₫, VND

### API Usage

```rust
// Simple functions
let tokens = text_to_tokens("Guten Tag", "de");
let phonemes = text_to_phonemes("olá mundo", "pt");
let tokens = text_to_tokens("안녕하세요", "ko");
let phonemes = text_to_phonemes("xin chào", "vi");

// Pipeline API
let mut pipeline = KPipeline::new("ko");
let result = pipeline.process("안녕하세요");
// result.phonemes: "annjʌŋhasejo"
// result.tokens: [0, 43, 56, 56, 82, 83, ...]

// Language codes supported:
// German: "de", "german", "deutsch"
// Portuguese: "pt", "pt-br", "pt-pt", "portuguese", "português"
// Korean: "ko", "korean", "한국어"
// Vietnamese: "vi", "vietnamese", "tiếng việt"
```

### Test Results

**173 tests passing** with `--features full`:
- German: 9 tests (umlauts, ch-variations, sch, diphthongs, final devoicing)
- Portuguese: 6 tests (nasal vowels, lh/nh digraphs, Brazilian palatalization)
- Korean: 5 tests (Hangul decomposition, basic conversion, hello)
- Vietnamese: 7 tests (tone detection, digraphs, d/đ distinction)

---

## Known Limitations / Future Work

### 1. Size Optimization (if needed)
- [ ] Compress English dictionaries with zstd
- [ ] Use minimal jieba dictionary (`default-features = false` + custom dict)
- [ ] Consider UPX compression for deployment

### 2. Polyphone Dictionary
- Current: ~100 phrase entries
- [ ] Expand to cover more common polyphones
- [ ] Consider loading from external JSON file for easier updates

### 3. UniFFI Bindings
- Feature flag added but not fully implemented
- [ ] Create `kokoro_g2p.udl` file
- [ ] Generate Swift/Kotlin bindings
- [ ] Test on iOS/Android

### 4. Traditional Chinese
- [ ] Add Traditional → Simplified conversion (optional)
- [ ] Consider zh-TW support

### 5. Mixed Language Text
- [ ] Handle Chinese text with embedded English
- [ ] Language detection for automatic switching

### 6. Phase 2 Languages ✅ COMPLETED
- [x] **German** (135M speakers) - Umlauts, ch-laut, final devoicing
- [x] **Portuguese** (264M speakers) - Brazilian/European variants, nasal vowels
- [x] **Korean** (80M speakers) - Hangul decomposition, jamo processing
- [x] **Vietnamese** (86M speakers) - 6 tones, Northern pronunciation

### 7. Phase 3 Languages (Recommended Next)
- [ ] **Hindi** (609M speakers) - Hard, schwa deletion
- [ ] **Russian** (255M speakers) - Hard, unpredictable stress
- [ ] **French** (321M speakers) - Hard, liaison rules

### 8. Phase 4 Languages (Major R&D)
- [ ] **Arabic** (422M speakers) - Very Hard, diacritization needed
- [ ] **Japanese** (125M speakers) - Very Hard, kanji readings
- [ ] **Thai** (65M speakers) - Very Hard, word segmentation

## Build Commands

```bash
# Development
cargo build --features chinese
cargo test --features chinese

# Release builds
cargo build --release --features chinese      # Chinese only
cargo build --release                         # English only (default)
cargo build --release --features full         # Both languages

# Check binary size
ls -lh target/release/kokoro_g2p.dll   # Windows
ls -lh target/release/libkokoro_g2p.so # Linux
ls -lh target/release/libkokoro_g2p.dylib # macOS
```

## File Locations

- Main implementation: `src/zh/`
- Tests: Inline in each module + `src/lib.rs`
- Configuration: `Cargo.toml`

## Dependencies Added

| Crate | Version | Purpose |
|-------|---------|---------|
| jieba-rs | 0.7 | Chinese word segmentation |
| uniffi | 0.28 | Mobile bindings (optional) |

## Contact / Questions

This implementation follows the patterns established in the existing English G2P code. Key design decisions:
1. Feature flags for conditional compilation
2. Lazy static initialization for dictionaries
3. PHF (perfect hash function) for static mappings
4. Reference-based lexicon access (zero-copy)

For questions about the implementation, review:
1. `src/zh/mod.rs` - Main entry point and pipeline
2. `src/pipeline.rs` - Unified multi-language interface
3. `src/lib.rs` - Language dispatch logic
