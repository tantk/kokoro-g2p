# G2P Validation Tests

This directory contains validation tests for the kokoro-g2p library using external pronunciation dictionaries.

## Data Sources

### WikiPron
[WikiPron](https://github.com/CUNY-CL/wikipron) is a multilingual pronunciation database mined from Wiktionary.

Downloaded files:
| File | Language | Entries |
|------|----------|---------|
| `eng_us_broad.tsv` | English (US) | 81K |
| `deu_broad.tsv` | German | 50K |
| `spa_la_broad.tsv` | Spanish (Latin America) | 99K |
| `por_br_broad.tsv` | Portuguese (Brazilian) | 139K |
| `ita_broad.tsv` | Italian | 80K |
| `tur_broad.tsv` | Turkish | 7K |
| `ind_broad.tsv` | Indonesian | 5K |
| `kor_narrow.tsv` | Korean | 26K |
| `vie_hanoi.tsv` | Vietnamese (Hanoi) | 23K |
| `zho_broad.tsv` | Chinese | 159K |

### espeak-ng (optional)
[espeak-ng](https://github.com/espeak-ng/espeak-ng) can be used as an additional reference for phoneme validation.

## Running Tests

### All Languages
```bash
cargo test --features full --test validation -- --nocapture
```

### English Only
```bash
# Common words test (recommended)
cargo test --features english --test validation test_validate_english_common -- --nocapture

# WikiPron sample test
cargo test --features english --test validation test_validate_english_sample -- --nocapture

# Extended test (1000 words)
cargo test --features english --test validation test_validate_english_extended -- --nocapture
```

### Specific Language
```bash
cargo test --features german --test validation test_validate_german -- --nocapture
cargo test --features spanish --test validation test_validate_spanish -- --nocapture
# etc.
```

## Results Summary

### English (Common Words)
| Metric | Value |
|--------|-------|
| Exact Match | 80% |
| Avg PER | 0.051 |

**Note**: WikiPron contains many rare/obscure words (acronyms, proper nouns, informal contractions) that are not in our dictionary. The common words test provides a more realistic accuracy measure.

### Phase 1 Languages (WikiPron sample)
| Language | Exact Match | Avg PER |
|----------|-------------|---------|
| Spanish | 78% | 0.046 |
| Italian | 49% | 0.134 |
| Indonesian | 36% | 0.254 |
| Turkish | 20% | 0.262 |

### Phase 2 Languages (WikiPron sample)
| Language | Exact Match | Avg PER |
|----------|-------------|---------|
| Portuguese | 16% | 0.283 |
| German | 6% | 0.363 |
| Korean | 0% | 0.618 |
| Vietnamese | 0% | 0.665 |

**Note**: Lower accuracy for Korean/Vietnamese is expected because:
1. WikiPron uses narrow phonetic transcription with precise tone markers
2. Our G2P uses simplified broad transcription optimized for TTS

## Understanding PER (Phoneme Error Rate)

PER = Levenshtein Distance / Reference Length

- **0.0**: Perfect match
- **< 0.1**: Excellent (minor notation differences)
- **< 0.3**: Good (acceptable for TTS)
- **< 0.5**: Fair (may need improvement)
- **> 0.5**: Poor (significant issues)

## IPA Normalization

The validation tests normalize IPA to handle notation differences:

| Our Notation | WikiPron | Meaning |
|--------------|----------|---------|
| ʧ | tʃ | voiceless affricate |
| ʤ | dʒ | voiced affricate |
| O | oʊ | diphthong |
| I | aɪ | diphthong |
| W | aʊ | diphthong |
| ᵊ | ə | schwa |
| ɾ | t | flap/tap |

## espeak-ng Validation (Optional)

If you have espeak-ng installed:

```bash
# Generate reference data
python tests/espeak_validation.py --all --save

# Check specific language
python tests/espeak_validation.py --lang de
```

Install espeak-ng:
- Windows: `winget install espeak-ng` or download from [releases](https://github.com/espeak-ng/espeak-ng/releases)
- macOS: `brew install espeak-ng`
- Linux: `sudo apt install espeak-ng`

## Updating WikiPron Data

To refresh the WikiPron data:

```bash
cd tests/wikipron

# Download specific language
curl -sL "https://raw.githubusercontent.com/CUNY-CL/wikipron/master/data/scrape/tsv/eng_latn_us_broad.tsv" -o eng_us_broad.tsv

# Available files: https://github.com/CUNY-CL/wikipron/tree/master/data/scrape/tsv
```

## Adding New Language Tests

1. Download WikiPron data for the language
2. Add a test function in `validation.rs`:

```rust
#[test]
#[cfg(feature = "newlang")]
fn test_validate_newlang_sample() {
    use kokoro_g2p::nl::NewLangG2P;

    let dir = get_test_data_dir();
    let path = dir.join("newlang_broad.tsv");
    // ... (see existing tests for pattern)
}
```

3. Run: `cargo test --features newlang --test validation test_validate_newlang -- --nocapture`
