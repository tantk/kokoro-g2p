//! Kokoro G2P - Grapheme-to-Phoneme Engine for Kokoro TTS
//!
//! This library converts text to phoneme token IDs suitable for the Kokoro TTS models.
//! It supports:
//! - English (American and British)
//! - Chinese (Mandarin, zh-CN) - requires `chinese` feature
//!
//! # Example
//!
//! ```rust
//! use kokoro_g2p::{text_to_tokens, text_to_phonemes};
//!
//! // Convert English text to token IDs
//! let tokens = text_to_tokens("Hello, world!", "en-us");
//! assert!(!tokens.is_empty());
//!
//! // Get intermediate phoneme representation
//! let phonemes = text_to_phonemes("Hello, world!", "en-us");
//! println!("Phonemes: {}", phonemes);
//! ```
//!
//! # Chinese Support
//!
//! Enable the `chinese` feature for Mandarin Chinese support:
//!
//! ```toml
//! [dependencies]
//! kokoro-g2p = { version = "0.1", features = ["chinese"] }
//! ```
//!
//! ```rust,ignore
//! use kokoro_g2p::{text_to_tokens, text_to_phonemes};
//!
//! // Convert Chinese text to tokens
//! let tokens = text_to_tokens("你好世界", "zh");
//! let phonemes = text_to_phonemes("你好世界", "zh");
//! ```

#[cfg(feature = "english")]
pub mod g2p;
#[cfg(feature = "english")]
pub mod lexicon;
#[cfg(feature = "english")]
pub mod preprocessor;

pub mod tokenizer;

#[cfg(feature = "chinese")]
pub mod zh;

#[cfg(feature = "japanese")]
pub mod ja;

#[cfg(feature = "spanish")]
pub mod es;

#[cfg(feature = "indonesian")]
pub mod id;

#[cfg(feature = "turkish")]
pub mod tr;

#[cfg(feature = "italian")]
pub mod it;

#[cfg(feature = "german")]
pub mod de;

#[cfg(feature = "portuguese")]
pub mod pt;

#[cfg(feature = "korean")]
pub mod ko;

#[cfg(feature = "vietnamese")]
pub mod vi;

pub mod pipeline;

/// Safely truncate a string at character boundary (not byte boundary)
/// Returns a substring containing at most `max_bytes` bytes, ending at a valid char boundary
pub fn safe_truncate(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    // Find the last valid character boundary at or before max_bytes
    let mut end = max_bytes.min(s.len());
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

// Re-export main functions
#[cfg(feature = "english")]
pub use g2p::{text_to_phoneme_string as text_to_phonemes_en, G2P};
pub use tokenizer::{phonemes_to_tokens, tokens_to_phonemes, MAX_TOKENS, PAD_TOKEN};
pub use pipeline::KPipeline;

/// Convert text to token IDs with automatic language detection or explicit language
///
/// Supported language codes:
/// - `"en"`, `"en-us"`, `"english"` - American English
/// - `"en-gb"`, `"british"` - British English
/// - `"zh"`, `"zh-cn"`, `"chinese"`, `"mandarin"` - Mandarin Chinese (requires `chinese` feature)
/// - `"es"`, `"spanish"` - Spanish (requires `spanish` feature)
/// - `"id"`, `"indonesian"` - Indonesian (requires `indonesian` feature)
/// - `"tr"`, `"turkish"` - Turkish (requires `turkish` feature)
/// - `"it"`, `"italian"` - Italian (requires `italian` feature)
pub fn text_to_tokens(text: &str, language: &str) -> Vec<i64> {
    let lang_lower = language.to_lowercase();

    log::debug!("text_to_tokens: lang='{}', text='{}'", &lang_lower, safe_truncate(&text, 30));

    match lang_lower.as_str() {
        #[cfg(feature = "chinese")]
        "zh" | "zh-cn" | "chinese" | "mandarin" | "cmn" => {
            log::debug!("Using Chinese G2P for: {}", safe_truncate(&text, 20));
            let tokens = zh::text_to_tokens(text);
            log::debug!("Chinese G2P returned {} tokens", tokens.len());
            tokens
        }
        #[cfg(feature = "japanese")]
        "ja" | "jp" | "japanese" | "日本語" => {
            log::debug!("Using Japanese G2P for: {}", safe_truncate(&text, 20));
            let tokens = ja::text_to_tokens(text);
            log::debug!("Japanese G2P returned {} tokens", tokens.len());
            tokens
        }
        #[cfg(feature = "spanish")]
        "es" | "es-es" | "es-mx" | "spanish" | "español" => {
            es::text_to_tokens(text)
        }
        #[cfg(feature = "indonesian")]
        "id" | "indonesian" | "bahasa" => {
            id::text_to_tokens(text)
        }
        #[cfg(feature = "turkish")]
        "tr" | "turkish" | "türkçe" => {
            tr::text_to_tokens(text)
        }
        #[cfg(feature = "italian")]
        "it" | "italian" | "italiano" => {
            it::text_to_tokens(text)
        }
        #[cfg(feature = "german")]
        "de" | "german" | "deutsch" => {
            de::text_to_tokens(text)
        }
        #[cfg(feature = "portuguese")]
        "pt" | "pt-br" | "pt-pt" | "portuguese" | "português" => {
            pt::text_to_tokens(text)
        }
        #[cfg(feature = "korean")]
        "ko" | "korean" | "한국어" => {
            ko::text_to_tokens(text)
        }
        #[cfg(feature = "vietnamese")]
        "vi" | "vietnamese" | "tiếng việt" => {
            vi::text_to_tokens(text)
        }
        #[cfg(feature = "english")]
        _ => {
            g2p::text_to_tokens(text, language)
        }
        #[cfg(not(feature = "english"))]
        _ => {
            log::warn!("Language '{}' not supported without 'english' feature", language);
            vec![PAD_TOKEN, PAD_TOKEN]
        }
    }
}

/// Convert text to phoneme string with automatic language detection or explicit language
///
/// Supported language codes:
/// - `"en"`, `"en-us"`, `"english"` - American English
/// - `"en-gb"`, `"british"` - British English
/// - `"zh"`, `"zh-cn"`, `"chinese"`, `"mandarin"` - Mandarin Chinese (requires `chinese` feature)
/// - `"es"`, `"spanish"` - Spanish (requires `spanish` feature)
/// - `"id"`, `"indonesian"` - Indonesian (requires `indonesian` feature)
/// - `"tr"`, `"turkish"` - Turkish (requires `turkish` feature)
/// - `"it"`, `"italian"` - Italian (requires `italian` feature)
/// - `"de"`, `"german"` - German (requires `german` feature)
/// - `"pt"`, `"portuguese"` - Portuguese (requires `portuguese` feature)
/// - `"ko"`, `"korean"` - Korean (requires `korean` feature)
/// - `"vi"`, `"vietnamese"` - Vietnamese (requires `vietnamese` feature)
pub fn text_to_phonemes(text: &str, language: &str) -> String {
    let lang_lower = language.to_lowercase();

    match lang_lower.as_str() {
        #[cfg(feature = "chinese")]
        "zh" | "zh-cn" | "chinese" | "mandarin" | "cmn" => {
            zh::text_to_phonemes(text)
        }
        #[cfg(feature = "japanese")]
        "ja" | "jp" | "japanese" | "日本語" => {
            ja::text_to_phonemes(text)
        }
        #[cfg(feature = "spanish")]
        "es" | "es-es" | "es-mx" | "spanish" | "español" => {
            es::text_to_phonemes(text)
        }
        #[cfg(feature = "indonesian")]
        "id" | "indonesian" | "bahasa" => {
            id::text_to_phonemes(text)
        }
        #[cfg(feature = "turkish")]
        "tr" | "turkish" | "türkçe" => {
            tr::text_to_phonemes(text)
        }
        #[cfg(feature = "italian")]
        "it" | "italian" | "italiano" => {
            it::text_to_phonemes(text)
        }
        #[cfg(feature = "german")]
        "de" | "german" | "deutsch" => {
            de::text_to_phonemes(text)
        }
        #[cfg(feature = "portuguese")]
        "pt" | "pt-br" | "pt-pt" | "portuguese" | "português" => {
            pt::text_to_phonemes(text)
        }
        #[cfg(feature = "korean")]
        "ko" | "korean" | "한국어" => {
            ko::text_to_phonemes(text)
        }
        #[cfg(feature = "vietnamese")]
        "vi" | "vietnamese" | "tiếng việt" => {
            vi::text_to_phonemes(text)
        }
        #[cfg(feature = "english")]
        _ => {
            g2p::text_to_phoneme_string(text, language)
        }
        #[cfg(not(feature = "english"))]
        _ => {
            log::warn!("Language '{}' not supported without 'english' feature", language);
            String::new()
        }
    }
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// ============================================================================
// JNI Interface for Android
// ============================================================================

#[cfg(feature = "jni")]
mod jni_interface {
    use jni::objects::{JClass, JString};
    use jni::sys::jlongArray;
    use jni::JNIEnv;
    use std::sync::Once;
    use crate::safe_truncate;

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            android_logger::init_once(
                android_logger::Config::default()
                    .with_max_level(log::LevelFilter::Debug)
                    .with_tag("KokoroG2P"),
            );
        });
    }

    /// JNI entry point for tokenization
    ///
    /// # Safety
    ///
    /// This function is called from Java/Kotlin via JNI.
    #[no_mangle]
    pub extern "system" fn Java_com_openvoice_app_engine_KokoroTokenizer_tokenize(
        mut env: JNIEnv,
        _class: JClass,
        text: JString,
    ) -> jlongArray {
        init_logger();

        // Get the text string from Java
        let text: String = match env.get_string(&text) {
            Ok(s) => s.into(),
            Err(e) => {
                log::error!("Failed to get string from JNI: {}", e);
                if let Ok(output) = env.new_long_array(2) {
                    let _ = env.set_long_array_region(&output, 0, &[0, 0]);
                    return output.into_raw();
                }
                return std::ptr::null_mut();
            }
        };

        // Perform tokenization using unified function
        let tokens = crate::text_to_tokens(&text, "en-us");

        // Create and populate the output array
        let output = match env.new_long_array(tokens.len() as i32) {
            Ok(arr) => arr,
            Err(e) => {
                log::error!("Failed to create output array: {}", e);
                if let Ok(output) = env.new_long_array(2) {
                    let _ = env.set_long_array_region(&output, 0, &[0, 0]);
                    return output.into_raw();
                }
                return std::ptr::null_mut();
            }
        };

        if let Err(e) = env.set_long_array_region(&output, 0, &tokens) {
            log::error!("Failed to set array region: {}", e);
        }

        output.into_raw()
    }

    /// JNI entry point for tokenization with language parameter
    #[no_mangle]
    pub extern "system" fn Java_com_openvoice_app_engine_KokoroTokenizer_tokenizeWithLanguage(
        mut env: JNIEnv,
        _class: JClass,
        text: JString,
        language: JString,
    ) -> jlongArray {
        init_logger();

        let text: String = match env.get_string(&text) {
            Ok(s) => s.into(),
            Err(_) => String::new(),
        };

        let language: String = match env.get_string(&language) {
            Ok(s) => s.into(),
            Err(_) => "en-us".to_string(),
        };

        log::debug!("tokenizeWithLanguage: text='{}', lang='{}'", safe_truncate(&text, 20), &language);

        // Use catch_unwind to prevent panics from crashing the app
        let tokens = std::panic::catch_unwind(|| {
            crate::text_to_tokens(&text, &language)
        }).unwrap_or_else(|e| {
            log::error!("Panic in text_to_tokens: {:?}", e);
            vec![0, 0]  // Return padding tokens on error
        });

        log::debug!("Result: {} tokens", tokens.len());

        match env.new_long_array(tokens.len() as i32) {
            Ok(output) => {
                let _ = env.set_long_array_region(&output, 0, &tokens);
                output.into_raw()
            }
            Err(e) => {
                log::error!("Failed to create output array: {:?}", e);
                if let Ok(output) = env.new_long_array(2) {
                    let _ = env.set_long_array_region(&output, 0, &[0, 0]);
                    return output.into_raw();
                }
                std::ptr::null_mut()
            }
        }
    }

    /// JNI entry point for checking enabled features (diagnostic)
    #[no_mangle]
    pub extern "system" fn Java_com_openvoice_app_engine_KokoroTokenizer_getEnabledFeatures<'a>(
        mut env: JNIEnv<'a>,
        _class: JClass,
    ) -> JString<'a> {
        init_logger();

        let mut features = Vec::new();

        #[cfg(feature = "english")]
        features.push("english");

        #[cfg(feature = "chinese")]
        features.push("chinese");

        #[cfg(feature = "japanese")]
        features.push("japanese");

        #[cfg(feature = "spanish")]
        features.push("spanish");

        #[cfg(feature = "indonesian")]
        features.push("indonesian");

        #[cfg(feature = "turkish")]
        features.push("turkish");

        #[cfg(feature = "italian")]
        features.push("italian");

        #[cfg(feature = "german")]
        features.push("german");

        #[cfg(feature = "portuguese")]
        features.push("portuguese");

        #[cfg(feature = "korean")]
        features.push("korean");

        #[cfg(feature = "vietnamese")]
        features.push("vietnamese");

        let result = features.join(",");
        log::info!("Enabled features: {}", &result);
        match env.new_string(&result) {
            Ok(s) => s,
            Err(_) => env.new_string("").expect("Failed to create empty JNI string"),
        }
    }

    /// JNI entry point for getting phoneme string (useful for debugging)
    #[no_mangle]
    pub extern "system" fn Java_com_openvoice_app_engine_KokoroTokenizer_textToPhonemes<'a>(
        mut env: JNIEnv<'a>,
        _class: JClass,
        text: JString,
        language: JString,
    ) -> JString<'a> {
        let text: String = match env.get_string(&text) {
            Ok(s) => s.into(),
            Err(_) => String::new(),
        };

        let language: String = match env.get_string(&language) {
            Ok(s) => s.into(),
            Err(_) => "en-us".to_string(),
        };

        // Use unified function that supports multiple languages
        let phonemes = crate::text_to_phonemes(&text, &language);

        match env.new_string(phonemes) {
            Ok(s) => s,
            Err(_) => env.new_string("").expect("Failed to create empty JNI string"),
        }
    }
}

// ============================================================================
// C FFI Interface (for iOS and other platforms)
// ============================================================================

/// C-compatible result structure
#[repr(C)]
pub struct CTokenArray {
    /// Pointer to token data
    pub data: *mut i64,
    /// Number of tokens
    pub len: usize,
    /// Capacity of the buffer
    pub capacity: usize,
}

/// Convert text to tokens (C API)
///
/// # Safety
///
/// The caller must ensure that:
/// - `text` is a valid null-terminated UTF-8 string
/// - `language` is a valid null-terminated UTF-8 string
/// - The returned `CTokenArray` must be freed using `kokoro_free_tokens`
#[no_mangle]
pub unsafe extern "C" fn kokoro_text_to_tokens(
    text: *const std::ffi::c_char,
    language: *const std::ffi::c_char,
) -> CTokenArray {
    if text.is_null() {
        return CTokenArray {
            data: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        };
    }

    let text = match std::ffi::CStr::from_ptr(text).to_str() {
        Ok(s) => s,
        Err(_) => {
            return CTokenArray {
                data: std::ptr::null_mut(),
                len: 0,
                capacity: 0,
            }
        }
    };

    let language = if language.is_null() {
        "en-us"
    } else {
        match std::ffi::CStr::from_ptr(language).to_str() {
            Ok(s) => s,
            Err(_) => "en-us",
        }
    };

    // Use unified function that supports multiple languages
    let mut tokens = text_to_tokens(text, language);
    let result = CTokenArray {
        data: tokens.as_mut_ptr(),
        len: tokens.len(),
        capacity: tokens.capacity(),
    };

    std::mem::forget(tokens);
    result
}

/// Free tokens allocated by `kokoro_text_to_tokens`
///
/// # Safety
///
/// The caller must ensure that `array` was returned by `kokoro_text_to_tokens`
/// and has not been freed before.
#[no_mangle]
pub unsafe extern "C" fn kokoro_free_tokens(array: CTokenArray) {
    if !array.data.is_null() {
        let _ = Vec::from_raw_parts(array.data, array.len, array.capacity);
    }
}

/// Get phoneme string (C API)
///
/// # Safety
///
/// The caller must ensure that:
/// - `text` and `language` are valid null-terminated UTF-8 strings
/// - The returned string must be freed using `kokoro_free_string`
#[no_mangle]
pub unsafe extern "C" fn kokoro_text_to_phonemes(
    text: *const std::ffi::c_char,
    language: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    if text.is_null() {
        return std::ptr::null_mut();
    }

    let text = match std::ffi::CStr::from_ptr(text).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let language = if language.is_null() {
        "en-us"
    } else {
        match std::ffi::CStr::from_ptr(language).to_str() {
            Ok(s) => s,
            Err(_) => "en-us",
        }
    };

    // Use unified function that supports multiple languages
    let phonemes = text_to_phonemes(text, language);

    match std::ffi::CString::new(phonemes) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free string allocated by `kokoro_text_to_phonemes`
///
/// # Safety
///
/// The caller must ensure that `s` was returned by `kokoro_text_to_phonemes`
/// and has not been freed before.
#[no_mangle]
pub unsafe extern "C" fn kokoro_free_string(s: *mut std::ffi::c_char) {
    if !s.is_null() {
        let _ = std::ffi::CString::from_raw(s);
    }
}

/// Get library version (C API)
#[no_mangle]
pub extern "C" fn kokoro_version() -> *const std::ffi::c_char {
    static VERSION_CSTR: once_cell::sync::Lazy<std::ffi::CString> =
        once_cell::sync::Lazy::new(|| std::ffi::CString::new(VERSION).unwrap());
    VERSION_CSTR.as_ptr()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "english")]
    fn test_text_to_tokens() {
        let tokens = text_to_tokens("Hello, world!", "en-us");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_text_to_phonemes() {
        let phonemes = text_to_phonemes("Hello, world!", "en-us");
        assert!(!phonemes.is_empty());
        println!("Phonemes: {}", phonemes);
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_hello_world() {
        let tokens = text_to_tokens("Hello, world!", "en");
        assert!(tokens.len() > 0);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_numbers() {
        let tokens = text_to_tokens("I have 3 apples.", "en");
        // Should tokenize "three" not "3"
        assert!(tokens.len() > 5);
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_max_length() {
        let long_text = "word ".repeat(200);
        let tokens = text_to_tokens(&long_text, "en");
        assert!(tokens.len() <= 512); // Max 510 + 2 padding
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_british_english() {
        let tokens_us = text_to_tokens("color", "en-us");
        let tokens_gb = text_to_tokens("colour", "en-gb");

        assert!(!tokens_us.is_empty());
        assert!(!tokens_gb.is_empty());
    }

    #[test]
    #[cfg(feature = "english")]
    fn test_roundtrip() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let tokens = text_to_tokens(text, "en");
        let phonemes = tokens_to_phonemes(&tokens);

        assert!(!phonemes.is_empty());
        assert!(phonemes.chars().all(|c| c != '❓'));
    }

    // ========================================================================
    // Chinese G2P Tests
    // ========================================================================

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_text_to_tokens() {
        let phonemes = text_to_phonemes("你好世界", "zh");
        println!("Chinese phonemes for '你好世界': {}", phonemes);

        let tokens = text_to_tokens("你好世界", "zh");
        println!("Chinese tokens for '你好世界': {:?} (count: {})", tokens, tokens.len());

        assert!(tokens.len() > 2, "Expected more than 2 tokens, got {}", tokens.len());
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_text_to_phonemes() {
        let phonemes = text_to_phonemes("你好", "zh");
        assert!(!phonemes.is_empty());
        // Chinese G2P outputs IPA phonemes with tone markers
        assert!(phonemes.chars().any(|c| c.is_alphabetic() || matches!(c, '↗' | '↘' | '↓' | '→')));
        println!("Chinese phonemes: {}", phonemes);
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_ni_hao_sandhi() {
        // 你好 should apply 3-3 tone sandhi
        let phonemes = text_to_phonemes("你好", "zh");
        // After sandhi, first syllable should have tone 2 marker (↗)
        println!("你好 phonemes: {}", phonemes);
        assert!(phonemes.contains('↗')); // Tone 2 marker for first syllable
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_yi_ge_sandhi() {
        // 一个 should apply 一 sandhi: yi1 + ge4 → yi2 + ge4
        let phonemes = text_to_phonemes("一个", "zh");
        println!("一个 phonemes: {}", phonemes);
        // Should have tone 2 marker for 一 before 4th tone
        assert!(phonemes.contains('↗'));
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_bu_shi_sandhi() {
        // 不是 should apply 不 sandhi: bu4 + shi4 → bu2 + shi4
        let phonemes = text_to_phonemes("不是", "zh");
        println!("不是 phonemes: {}", phonemes);
        // Should have tone 2 marker for 不 before 4th tone
        assert!(phonemes.contains('↗'));
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_polyphone_xing() {
        // 行走 (xíng zǒu) vs 银行 (yín háng)
        let phonemes1 = text_to_phonemes("行走", "zh");
        let phonemes2 = text_to_phonemes("银行", "zh");
        println!("行走 phonemes: {}", phonemes1);
        println!("银行 phonemes: {}", phonemes2);
        // They should be different
        assert_ne!(phonemes1, phonemes2);
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_currency_sgd() {
        // S$100 → 新加坡元一百
        let phonemes = text_to_phonemes("S$100", "zh");
        println!("S$100 phonemes: {}", phonemes);
        // Should have converted to Chinese spoken form
        assert!(!phonemes.is_empty());
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_language_codes() {
        // Test various Chinese language codes
        let tokens_zh = text_to_tokens("你好", "zh");
        let tokens_zh_cn = text_to_tokens("你好", "zh-cn");
        let tokens_chinese = text_to_tokens("你好", "chinese");
        let tokens_mandarin = text_to_tokens("你好", "mandarin");

        // All should produce the same result
        assert_eq!(tokens_zh, tokens_zh_cn);
        assert_eq!(tokens_zh, tokens_chinese);
        assert_eq!(tokens_zh, tokens_mandarin);
    }

    #[test]
    #[cfg(feature = "chinese")]
    fn test_chinese_pipeline() {
        let mut pipeline = KPipeline::new("zh");
        let result = pipeline.process("你好世界");
        assert!(!result.phonemes.is_empty());
        assert!(result.tokens.len() > 2);
    }

    // ========================================================================
    // Spanish G2P Tests
    // ========================================================================

    #[test]
    #[cfg(feature = "spanish")]
    fn test_spanish_text_to_tokens() {
        let tokens = text_to_tokens("hola mundo", "es");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    #[cfg(feature = "spanish")]
    fn test_spanish_text_to_phonemes() {
        let phonemes = text_to_phonemes("hola mundo", "es");
        assert!(!phonemes.is_empty());
        println!("Spanish phonemes: {}", phonemes);
    }

    #[test]
    #[cfg(feature = "spanish")]
    fn test_spanish_language_codes() {
        let tokens_es = text_to_tokens("hola", "es");
        let tokens_spanish = text_to_tokens("hola", "spanish");
        assert_eq!(tokens_es, tokens_spanish);
    }

    #[test]
    #[cfg(feature = "korean")]
    fn test_korean_tokens_in_range() {
        let text = "안녕하세요";
        let phonemes = text_to_phonemes(text, "ko");
        let tokens = text_to_tokens(text, "ko");
        let max_token = tokens.iter().max().copied().unwrap_or(0);

        println!("Korean '{}' -> phonemes: {}", text, phonemes);
        println!("Korean tokens: {:?}", tokens);
        println!("Max token ID: {} (must be <= 177)", max_token);

        assert!(max_token <= 177, "Token {} exceeds English vocab size 177", max_token);
    }

    // ========================================================================
    // Indonesian G2P Tests
    // ========================================================================

    #[test]
    #[cfg(feature = "indonesian")]
    fn test_indonesian_text_to_tokens() {
        let tokens = text_to_tokens("selamat pagi", "id");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    #[cfg(feature = "indonesian")]
    fn test_indonesian_text_to_phonemes() {
        let phonemes = text_to_phonemes("selamat pagi", "id");
        assert!(!phonemes.is_empty());
        println!("Indonesian phonemes: {}", phonemes);
    }

    #[test]
    #[cfg(feature = "indonesian")]
    fn test_indonesian_language_codes() {
        let tokens_id = text_to_tokens("halo", "id");
        let tokens_indonesian = text_to_tokens("halo", "indonesian");
        assert_eq!(tokens_id, tokens_indonesian);
    }

    // ========================================================================
    // Turkish G2P Tests
    // ========================================================================

    #[test]
    #[cfg(feature = "turkish")]
    fn test_turkish_text_to_tokens() {
        let tokens = text_to_tokens("merhaba", "tr");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    #[cfg(feature = "turkish")]
    fn test_turkish_text_to_phonemes() {
        let phonemes = text_to_phonemes("merhaba dünya", "tr");
        assert!(!phonemes.is_empty());
        println!("Turkish phonemes: {}", phonemes);
    }

    #[test]
    #[cfg(feature = "turkish")]
    fn test_turkish_language_codes() {
        let tokens_tr = text_to_tokens("merhaba", "tr");
        let tokens_turkish = text_to_tokens("merhaba", "turkish");
        assert_eq!(tokens_tr, tokens_turkish);
    }

    // ========================================================================
    // Italian G2P Tests
    // ========================================================================

    #[test]
    #[cfg(feature = "italian")]
    fn test_italian_text_to_tokens() {
        let tokens = text_to_tokens("ciao mondo", "it");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    #[cfg(feature = "italian")]
    fn test_italian_text_to_phonemes() {
        let phonemes = text_to_phonemes("ciao mondo", "it");
        assert!(!phonemes.is_empty());
        println!("Italian phonemes: {}", phonemes);
    }

    #[test]
    #[cfg(feature = "italian")]
    fn test_italian_language_codes() {
        let tokens_it = text_to_tokens("ciao", "it");
        let tokens_italian = text_to_tokens("ciao", "italian");
        assert_eq!(tokens_it, tokens_italian);
    }
}
