/**
 * Kokoro G2P - C Header for iOS/Swift integration
 *
 * This header provides the C API for the Kokoro G2P library.
 * Use this with Swift via a bridging header.
 */

#ifndef KOKORO_G2P_H
#define KOKORO_G2P_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Token array structure returned by kokoro_text_to_tokens.
 */
typedef struct {
    /** Pointer to token data (i64 values) */
    int64_t *data;
    /** Number of tokens */
    size_t len;
    /** Capacity of the buffer */
    size_t capacity;
} CTokenArray;

/**
 * Convert text to phoneme token IDs.
 *
 * @param text Null-terminated UTF-8 text string
 * @param language Language code (e.g., "en-us", "en-gb"), or NULL for default
 * @return CTokenArray containing token IDs. Must be freed with kokoro_free_tokens.
 */
CTokenArray kokoro_text_to_tokens(const char *text, const char *language);

/**
 * Free tokens allocated by kokoro_text_to_tokens.
 *
 * @param array The token array to free
 */
void kokoro_free_tokens(CTokenArray array);

/**
 * Convert text to phoneme string.
 *
 * @param text Null-terminated UTF-8 text string
 * @param language Language code (e.g., "en-us", "en-gb"), or NULL for default
 * @return Phoneme string. Must be freed with kokoro_free_string.
 */
char *kokoro_text_to_phonemes(const char *text, const char *language);

/**
 * Free string allocated by kokoro_text_to_phonemes.
 *
 * @param s The string to free
 */
void kokoro_free_string(char *s);

/**
 * Get the library version.
 *
 * @return Version string (do not free)
 */
const char *kokoro_version(void);

#ifdef __cplusplus
}
#endif

#endif /* KOKORO_G2P_H */
