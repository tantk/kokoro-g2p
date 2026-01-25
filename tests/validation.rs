//! WikiPron validation tests for G2P accuracy
//!
//! This module loads WikiPron pronunciation dictionaries and validates
//! our G2P output against them.
//!
//! Note: WikiPron uses space-separated IPA, our G2P outputs continuous IPA.
//! We compare normalized versions and report accuracy metrics.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Load WikiPron TSV file (word\tIPA with spaces)
fn load_wikipron(path: &Path, limit: usize) -> Vec<(String, String)> {
    load_wikipron_filtered(path, limit, false)
}

/// Load WikiPron TSV file with optional filtering of edge cases
fn load_wikipron_filtered(path: &Path, limit: usize, filter_edge_cases: bool) -> Vec<(String, String)> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let word = parts[0].to_string();
                let ipa = parts[1].replace(' ', "");

                // Skip edge cases if filtering is enabled
                if filter_edge_cases {
                    // Skip words starting with apostrophe (informal contractions)
                    if word.starts_with('\'') || word.starts_with('\u{2019}') {
                        return None;
                    }
                    // Skip words with spaces (phrases)
                    if word.contains(' ') {
                        return None;
                    }
                    // Skip very short words
                    if word.len() < 2 {
                        return None;
                    }
                    // Skip words with numbers
                    if word.chars().any(|c| c.is_ascii_digit()) {
                        return None;
                    }
                    // Skip all-uppercase words (acronyms)
                    if word.chars().all(|c| c.is_ascii_uppercase() || !c.is_alphabetic()) {
                        return None;
                    }
                    // Skip words starting with uppercase (proper nouns) for English
                    if word.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false) {
                        return None;
                    }
                }

                Some((word, ipa))
            } else {
                None
            }
        })
        .take(limit)
        .collect()
}

/// Normalize IPA for comparison (remove stress marks, length marks, handle equivalents)
fn normalize_ipa(ipa: &str) -> String {
    let mut result: String = ipa.chars()
        .filter(|c| !matches!(c, 'ˈ' | 'ˌ' | 'ː' | '.' | '→' | '↗' | '↘' | '↓'))
        .collect();

    // Handle IPA notation equivalents
    result = result
        // Affricates
        .replace("tʃ", "ʧ")
        .replace("dʒ", "ʤ")
        // Diphthongs (simplified)
        .replace("oʊ", "O")
        .replace("eɪ", "A")
        .replace("aɪ", "I")
        .replace("aʊ", "W")
        .replace("ɔɪ", "Y")
        // Vowel variants
        .replace("ᵊ", "ə")
        .replace("ɚ", "əɹ")
        .replace("ɝ", "ɜɹ")
        // Flap
        .replace("ɾ", "t");

    result
}

/// Calculate Levenshtein distance between two strings
fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 { return n; }
    if n == 0 { return m; }

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
            dp[i][j] = (dp[i-1][j] + 1)
                .min(dp[i][j-1] + 1)
                .min(dp[i-1][j-1] + cost);
        }
    }

    dp[m][n]
}

/// Phoneme Error Rate (PER) - edit distance / reference length
fn phoneme_error_rate(predicted: &str, reference: &str) -> f64 {
    let pred_norm = normalize_ipa(predicted);
    let ref_norm = normalize_ipa(reference);

    if ref_norm.is_empty() {
        return if pred_norm.is_empty() { 0.0 } else { 1.0 };
    }

    let distance = levenshtein(&pred_norm, &ref_norm);
    distance as f64 / ref_norm.chars().count() as f64
}

/// Validation results for a language
#[derive(Debug)]
pub struct ValidationResult {
    pub language: String,
    pub total: usize,
    pub exact_matches: usize,
    pub avg_per: f64,
    pub samples: Vec<(String, String, String, f64)>, // word, expected, got, PER
}

impl ValidationResult {
    pub fn accuracy(&self) -> f64 {
        if self.total == 0 { 0.0 } else { self.exact_matches as f64 / self.total as f64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_dir() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("wikipron")
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("hello", "hello"), 0);
        assert_eq!(levenshtein("hello", "hallo"), 1);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
    }

    #[test]
    fn test_per() {
        assert!((phoneme_error_rate("hello", "hello") - 0.0).abs() < 0.001);
        assert!((phoneme_error_rate("hallo", "hello") - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_load_wikipron() {
        let dir = get_test_data_dir();
        let deu_path = dir.join("deu_broad.tsv");

        if deu_path.exists() {
            let entries = load_wikipron(&deu_path, 10);
            assert!(!entries.is_empty(), "Should load some German entries");
            println!("Sample German entries:");
            for (word, ipa) in entries.iter().take(5) {
                println!("  {} → {}", word, ipa);
            }
        }
    }

    #[test]
    #[cfg(feature = "german")]
    fn test_validate_german_sample() {
        use kokoro_g2p::de::GermanG2P;

        let dir = get_test_data_dir();
        let path = dir.join("deu_broad.tsv");

        if !path.exists() {
            println!("WikiPron German data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = GermanG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nGerman Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "spanish")]
    fn test_validate_spanish_sample() {
        use kokoro_g2p::es::SpanishG2P;

        let dir = get_test_data_dir();
        let path = dir.join("spa_la_broad.tsv");

        if !path.exists() {
            println!("WikiPron Spanish data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = SpanishG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nSpanish Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "korean")]
    fn test_validate_korean_sample() {
        use kokoro_g2p::ko::KoreanG2P;

        let dir = get_test_data_dir();
        let path = dir.join("kor_narrow.tsv");

        if !path.exists() {
            println!("WikiPron Korean data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = KoreanG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nKorean Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "vietnamese")]
    fn test_validate_vietnamese_sample() {
        use kokoro_g2p::vi::VietnameseG2P;

        let dir = get_test_data_dir();
        let path = dir.join("vie_hanoi.tsv");

        if !path.exists() {
            println!("WikiPron Vietnamese data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = VietnameseG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nVietnamese Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "portuguese")]
    fn test_validate_portuguese_sample() {
        use kokoro_g2p::pt::PortugueseG2P;

        let dir = get_test_data_dir();
        let path = dir.join("por_br_broad.tsv");

        if !path.exists() {
            println!("WikiPron Portuguese data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = PortugueseG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nPortuguese Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "indonesian")]
    fn test_validate_indonesian_sample() {
        use kokoro_g2p::id::IndonesianG2P;

        let dir = get_test_data_dir();
        let path = dir.join("ind_broad.tsv");

        if !path.exists() {
            println!("WikiPron Indonesian data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = IndonesianG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nIndonesian Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "turkish")]
    fn test_validate_turkish_sample() {
        use kokoro_g2p::tr::TurkishG2P;

        let dir = get_test_data_dir();
        let path = dir.join("tur_broad.tsv");

        if !path.exists() {
            println!("WikiPron Turkish data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = TurkishG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nTurkish Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "italian")]
    fn test_validate_italian_sample() {
        use kokoro_g2p::it::ItalianG2P;

        let dir = get_test_data_dir();
        let path = dir.join("ita_broad.tsv");

        if !path.exists() {
            println!("WikiPron Italian data not found, skipping validation");
            return;
        }

        let entries = load_wikipron(&path, 100);
        let g2p = ItalianG2P::new();

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nItalian Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_validate_english_sample() {
        use kokoro_g2p::G2P;

        let dir = get_test_data_dir();
        let path = dir.join("eng_us_broad.tsv");

        if !path.exists() {
            println!("WikiPron English data not found, skipping validation");
            return;
        }

        // Use filtered loading to skip edge cases like 'Murica, 'bout, etc.
        let entries = load_wikipron_filtered(&path, 100, true);
        let g2p = G2P::new(false); // US English

        let mut total_per = 0.0;
        let mut exact = 0;
        let mut samples: Vec<(String, String, String, f64)> = Vec::new();

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }

            // Save samples for debugging (first 10 with PER > 0)
            if samples.len() < 10 && per > 0.0 {
                samples.push((word.clone(), expected.clone(), predicted.clone(), per));
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nEnglish (US) Validation Results:");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);

        println!("\nSample mismatches:");
        for (word, expected, got, per) in &samples {
            println!("  {} → expected: {}, got: {}, PER: {:.2}", word, expected, got, per);
        }
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_validate_english_extended() {
        use kokoro_g2p::G2P;

        let dir = get_test_data_dir();
        let path = dir.join("eng_us_broad.tsv");

        if !path.exists() {
            println!("WikiPron English data not found, skipping validation");
            return;
        }

        // Test larger sample with filtering
        let entries = load_wikipron_filtered(&path, 1000, true);
        let g2p = G2P::new(false);

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &entries {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            if normalize_ipa(&predicted) == normalize_ipa(expected) {
                exact += 1;
            }
        }

        let avg_per = total_per / entries.len() as f64;
        let accuracy = exact as f64 / entries.len() as f64;

        println!("\nEnglish Extended Validation (1000 words):");
        println!("  Total: {}", entries.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }

    /// Test with common English words to get a realistic accuracy measurement
    #[test]
    #[cfg(feature = "english")]
    fn test_validate_english_common_words() {
        use kokoro_g2p::G2P;

        // Common English words with expected IPA (CMU-style simplified)
        let test_cases = [
            ("hello", "hɛloʊ"),
            ("world", "wɝld"),
            ("the", "ðə"),
            ("computer", "kəmpjutɚ"),
            ("beautiful", "bjutɪfəl"),
            ("language", "læŋɡwɪdʒ"),
            ("science", "saɪəns"),
            ("people", "pipəl"),
            ("about", "əbaʊt"),
            ("time", "taɪm"),
            ("water", "wɔtɚ"),
            ("music", "mjuzɪk"),
            ("family", "fæməli"),
            ("important", "ɪmpɔɹtənt"),
            ("different", "dɪfɹənt"),
            ("country", "kʌntɹi"),
            ("question", "kwɛstʃən"),
            ("school", "skul"),
            ("children", "tʃɪldɹən"),
            ("example", "ɪɡzæmpəl"),
        ];

        let g2p = G2P::new(false);

        println!("\nCommon English Words Validation:");
        println!("{:-<60}", "");

        let mut total_per = 0.0;
        let mut exact = 0;

        for (word, expected) in &test_cases {
            let predicted = g2p.text_to_phonemes(word);
            let per = phoneme_error_rate(&predicted, expected);
            total_per += per;

            let normalized_pred = normalize_ipa(&predicted);
            let normalized_exp = normalize_ipa(expected);

            if normalized_pred == normalized_exp {
                exact += 1;
                println!("  {} → {} [MATCH]", word, predicted);
            } else {
                println!("  {} → {} (expected: {}, PER: {:.2})", word, predicted, expected, per);
            }
        }

        let avg_per = total_per / test_cases.len() as f64;
        let accuracy = exact as f64 / test_cases.len() as f64;

        println!("{:-<60}", "");
        println!("  Total: {}", test_cases.len());
        println!("  Exact matches: {} ({:.1}%)", exact, accuracy * 100.0);
        println!("  Avg PER: {:.3}", avg_per);
    }
}
