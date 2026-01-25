/**
 * Kokoro G2P Tokenizer for Android
 *
 * Converts English text to phoneme token IDs for the Kokoro TTS model.
 *
 * Usage:
 * ```kotlin
 * val tokens = KokoroTokenizer.tokenize("Hello, world!")
 * // tokens is a LongArray of token IDs
 * ```
 */
package com.openvoice.app.engine

object KokoroTokenizer {

    init {
        System.loadLibrary("kokoro_g2p")
    }

    /**
     * Convert text to token IDs using American English pronunciation.
     *
     * @param text The input text to tokenize
     * @return Array of token IDs (i64) padded with 0 at start and end
     */
    external fun tokenize(text: String): LongArray

    /**
     * Convert text to token IDs with specified language variant.
     *
     * @param text The input text to tokenize
     * @param language Language code: "en-us" for American, "en-gb" for British
     * @return Array of token IDs (i64) padded with 0 at start and end
     */
    external fun tokenizeWithLanguage(text: String, language: String): LongArray

    /**
     * Get the phoneme representation of text (for debugging).
     *
     * @param text The input text
     * @param language Language code: "en-us" for American, "en-gb" for British
     * @return Phoneme string in IPA-like notation
     */
    external fun textToPhonemes(text: String, language: String): String

    /**
     * Convert text to token IDs with default settings.
     * Convenience wrapper around [tokenize].
     */
    fun convert(text: String): LongArray = tokenize(text)

    /**
     * Convert text to token IDs with specified language.
     * Convenience wrapper around [tokenizeWithLanguage].
     */
    fun convert(text: String, british: Boolean): LongArray {
        return tokenizeWithLanguage(text, if (british) "en-gb" else "en-us")
    }
}

/**
 * Extension function for easy tokenization of strings.
 */
fun String.toKokoroTokens(): LongArray = KokoroTokenizer.tokenize(this)

/**
 * Extension function for easy phoneme conversion of strings.
 */
fun String.toKokoroPhonemes(language: String = "en-us"): String =
    KokoroTokenizer.textToPhonemes(this, language)
