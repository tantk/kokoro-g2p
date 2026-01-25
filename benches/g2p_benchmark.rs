//! Benchmarks for the G2P engine

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kokoro_g2p::{text_to_phonemes, text_to_tokens};

fn benchmark_short_text(c: &mut Criterion) {
    c.bench_function("short_text_to_tokens", |b| {
        b.iter(|| text_to_tokens(black_box("Hello, world!"), "en-us"))
    });
}

fn benchmark_medium_text(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog. This is a test of the text-to-speech system.";
    c.bench_function("medium_text_to_tokens", |b| {
        b.iter(|| text_to_tokens(black_box(text), "en-us"))
    });
}

fn benchmark_long_text(c: &mut Criterion) {
    let text = "In the beginning, there was nothing but darkness and void. Then, a spark of light emerged, illuminating the cosmos and bringing forth the universe as we know it. Stars were born, galaxies formed, and planets coalesced from cosmic dust. On one small blue planet, life began its incredible journey from simple organisms to complex beings capable of contemplating their own existence.";

    c.bench_function("long_text_to_tokens", |b| {
        b.iter(|| text_to_tokens(black_box(text), "en-us"))
    });
}

fn benchmark_numbers(c: &mut Criterion) {
    let text = "I have 1234 dollars and 56 cents. The year is 2024.";
    c.bench_function("numbers_to_tokens", |b| {
        b.iter(|| text_to_tokens(black_box(text), "en-us"))
    });
}

fn benchmark_phoneme_conversion(c: &mut Criterion) {
    c.bench_function("text_to_phonemes", |b| {
        b.iter(|| text_to_phonemes(black_box("Hello, world!"), "en-us"))
    });
}

criterion_group!(
    benches,
    benchmark_short_text,
    benchmark_medium_text,
    benchmark_long_text,
    benchmark_numbers,
    benchmark_phoneme_conversion
);
criterion_main!(benches);
