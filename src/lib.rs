//! Kokoro G2P - Grapheme-to-Phoneme Engine for Kokoro TTS
//!
//! This library converts English text to phoneme token IDs suitable for
//! the Kokoro-82M TTS model. It supports both American and British English.
//!
//! # Example
//!
//! ```rust
//! use kokoro_g2p::{text_to_tokens, text_to_phonemes};
//!
//! // Convert text to token IDs for TTS model
//! let tokens = text_to_tokens("Hello, world!", "en-us");
//! assert!(!tokens.is_empty());
//!
//! // Get intermediate phoneme representation
//! let phonemes = text_to_phonemes("Hello, world!", "en-us");
//! println!("Phonemes: {}", phonemes);
//! ```

pub mod g2p;
pub mod lexicon;
pub mod preprocessor;
pub mod tokenizer;

// Re-export main functions
pub use g2p::{text_to_tokens, text_to_phoneme_string as text_to_phonemes, G2P};
pub use tokenizer::{phonemes_to_tokens, tokens_to_phonemes, MAX_TOKENS, PAD_TOKEN};

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
        // Get the text string from Java
        let text: String = match env.get_string(&text) {
            Ok(s) => s.into(),
            Err(e) => {
                log::error!("Failed to get string from JNI: {}", e);
                let output = env.new_long_array(2).unwrap();
                env.set_long_array_region(&output, 0, &[0, 0]).unwrap();
                return output;
            }
        };

        // Perform tokenization
        let tokens = crate::g2p::text_to_tokens(&text, "en-us");

        // Create and populate the output array
        let output = match env.new_long_array(tokens.len() as i32) {
            Ok(arr) => arr,
            Err(e) => {
                log::error!("Failed to create output array: {}", e);
                let output = env.new_long_array(2).unwrap();
                env.set_long_array_region(&output, 0, &[0, 0]).unwrap();
                return output;
            }
        };

        if let Err(e) = env.set_long_array_region(&output, 0, &tokens) {
            log::error!("Failed to set array region: {}", e);
        }

        output
    }

    /// JNI entry point for tokenization with language parameter
    #[no_mangle]
    pub extern "system" fn Java_com_openvoice_app_engine_KokoroTokenizer_tokenizeWithLanguage(
        mut env: JNIEnv,
        _class: JClass,
        text: JString,
        language: JString,
    ) -> jlongArray {
        let text: String = match env.get_string(&text) {
            Ok(s) => s.into(),
            Err(_) => String::new(),
        };

        let language: String = match env.get_string(&language) {
            Ok(s) => s.into(),
            Err(_) => "en-us".to_string(),
        };

        let tokens = crate::g2p::text_to_tokens(&text, &language);

        let output = env.new_long_array(tokens.len() as i32).unwrap();
        env.set_long_array_region(&output, 0, &tokens).unwrap();
        output
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

        let phonemes = crate::g2p::text_to_phoneme_string(&text, &language);

        env.new_string(phonemes).unwrap()
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

    let mut tokens = g2p::text_to_tokens(text, language);
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

    let phonemes = g2p::text_to_phoneme_string(text, language);

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
    fn test_text_to_tokens() {
        let tokens = text_to_tokens("Hello, world!", "en-us");
        assert!(tokens.len() > 2);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    fn test_text_to_phonemes() {
        let phonemes = text_to_phonemes("Hello, world!", "en-us");
        assert!(!phonemes.is_empty());
        println!("Phonemes: {}", phonemes);
    }

    #[test]
    fn test_hello_world() {
        let tokens = text_to_tokens("Hello, world!", "en");
        assert!(tokens.len() > 0);
        assert_eq!(tokens[0], PAD_TOKEN);
        assert_eq!(*tokens.last().unwrap(), PAD_TOKEN);
    }

    #[test]
    fn test_numbers() {
        let tokens = text_to_tokens("I have 3 apples.", "en");
        // Should tokenize "three" not "3"
        assert!(tokens.len() > 5);
    }

    #[test]
    fn test_max_length() {
        let long_text = "word ".repeat(200);
        let tokens = text_to_tokens(&long_text, "en");
        assert!(tokens.len() <= 512); // Max 510 + 2 padding
    }

    #[test]
    fn test_british_english() {
        let tokens_us = text_to_tokens("color", "en-us");
        let tokens_gb = text_to_tokens("colour", "en-gb");

        assert!(!tokens_us.is_empty());
        assert!(!tokens_gb.is_empty());
    }

    #[test]
    fn test_roundtrip() {
        let text = "The quick brown fox jumps over the lazy dog.";
        let tokens = text_to_tokens(text, "en");
        let phonemes = tokens_to_phonemes(&tokens);

        assert!(!phonemes.is_empty());
        assert!(phonemes.chars().all(|c| c != '❓'));
    }
}
