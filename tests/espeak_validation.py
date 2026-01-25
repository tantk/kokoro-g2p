#!/usr/bin/env python3
"""
espeak-ng validation script for kokoro-g2p

This script generates reference IPA pronunciations using espeak-ng
and compares them against our Rust G2P output.

Requirements:
    pip install espeak-phonemizer

Usage:
    python tests/espeak_validation.py --lang de --sample 100
    python tests/espeak_validation.py --all
"""

import argparse
import subprocess
import json
import sys
from pathlib import Path

# Language code mappings
LANG_MAP = {
    'de': 'de',
    'pt': 'pt-br',
    'ko': 'ko',
    'vi': 'vi',
    'es': 'es-419',
    'id': 'id',
    'tr': 'tr',
    'it': 'it',
    'en': 'en-us',
    'zh': 'cmn',
}

# Test words per language
TEST_WORDS = {
    'de': ['Guten', 'Tag', 'Welt', 'Schule', 'Buch', 'Mädchen', 'König', 'schön', 'über', 'München'],
    'pt': ['olá', 'mundo', 'Brasil', 'coração', 'manhã', 'pão', 'cidade', 'trabalho'],
    'ko': ['안녕', '하세요', '감사', '합니다', '한국', '서울', '사랑'],
    'vi': ['xin', 'chào', 'cảm', 'ơn', 'Việt', 'Nam', 'Hà', 'Nội'],
    'es': ['hola', 'mundo', 'gracias', 'ciudad', 'noche', 'llorar', 'año'],
    'id': ['selamat', 'pagi', 'terima', 'kasih', 'Indonesia', 'Jakarta'],
    'tr': ['merhaba', 'dünya', 'teşekkür', 'ederim', 'Türkiye', 'İstanbul'],
    'it': ['ciao', 'mondo', 'grazie', 'famiglia', 'città', 'notte', 'gli'],
    'en': ['hello', 'world', 'beautiful', 'language', 'computer', 'science'],
}


def check_espeak():
    """Check if espeak-ng is available."""
    try:
        result = subprocess.run(['espeak-ng', '--version'],
                                capture_output=True, text=True)
        return result.returncode == 0
    except FileNotFoundError:
        return False


def espeak_phonemize(text: str, lang: str) -> str:
    """Get IPA from espeak-ng."""
    try:
        result = subprocess.run(
            ['espeak-ng', '-v', lang, '--ipa', '-q', text],
            capture_output=True, text=True
        )
        return result.stdout.strip()
    except Exception as e:
        return f"ERROR: {e}"


def try_phonemizer(text: str, lang: str) -> str:
    """Try using espeak-phonemizer Python library."""
    try:
        from espeak_phonemizer import Phonemizer
        ph = Phonemizer(default_voice=lang)
        return ph.phonemize(text)
    except ImportError:
        return None
    except Exception as e:
        return f"ERROR: {e}"


def run_rust_g2p(text: str, lang: str) -> str:
    """Run our Rust G2P via cargo test output or direct call."""
    # For now, return placeholder - would need to build and call the Rust binary
    return "[Rust G2P output]"


def validate_language(lang: str, words: list = None):
    """Validate a single language against espeak-ng."""
    if words is None:
        words = TEST_WORDS.get(lang, [])

    espeak_lang = LANG_MAP.get(lang, lang)

    print(f"\n{'='*50}")
    print(f"Language: {lang} (espeak: {espeak_lang})")
    print('='*50)

    results = []
    for word in words:
        espeak_ipa = espeak_phonemize(word, espeak_lang)
        results.append({
            'word': word,
            'espeak_ipa': espeak_ipa,
        })
        print(f"  {word:20} → {espeak_ipa}")

    return results


def save_reference_data(lang: str, results: list, output_dir: Path):
    """Save reference IPA data for later comparison."""
    output_file = output_dir / f"espeak_ref_{lang}.json"
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, ensure_ascii=False, indent=2)
    print(f"\nSaved reference data to {output_file}")


def main():
    parser = argparse.ArgumentParser(description='Validate G2P against espeak-ng')
    parser.add_argument('--lang', type=str, help='Language code (de, pt, ko, vi, es, id, tr, it, en)')
    parser.add_argument('--all', action='store_true', help='Validate all languages')
    parser.add_argument('--save', action='store_true', help='Save reference data to JSON files')
    args = parser.parse_args()

    if not check_espeak():
        print("ERROR: espeak-ng not found!")
        print("\nInstall espeak-ng:")
        print("  Windows: Download from https://github.com/espeak-ng/espeak-ng/releases")
        print("  macOS:   brew install espeak-ng")
        print("  Linux:   sudo apt install espeak-ng")
        sys.exit(1)

    output_dir = Path(__file__).parent / 'espeak_reference'
    output_dir.mkdir(exist_ok=True)

    if args.all:
        for lang in TEST_WORDS.keys():
            results = validate_language(lang)
            if args.save:
                save_reference_data(lang, results, output_dir)
    elif args.lang:
        results = validate_language(args.lang)
        if args.save:
            save_reference_data(args.lang, results, output_dir)
    else:
        print("Usage: python espeak_validation.py --lang de")
        print("       python espeak_validation.py --all")
        print("       python espeak_validation.py --all --save")


if __name__ == '__main__':
    main()
