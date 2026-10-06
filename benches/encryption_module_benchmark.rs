use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

// Reuse the binary's module tree without introducing a library target.
#[path = "../src/encryption/mod.rs"]
mod encryption;

fn text_ciphers(c: &mut Criterion) {
    for (name, pattern) in [
        ("ascii", "Attack at dawn! "),
        ("unicode", "Análise 🦀 中! "),
    ] {
        let mut group = c.benchmark_group(format!("text/{name}"));
        for repeats in [0, 4, 256, 4096] {
            let text = pattern.repeat(repeats);
            let encrypted = encryption::vigenere_cipher(&text, "LeMoN", true);
            group.throughput(Throughput::Bytes(text.len() as u64));
            group.bench_with_input(BenchmarkId::new("caesar", text.len()), &text, |b, text| {
                b.iter(|| encryption::caesar_cipher(black_box(text), black_box(7)));
            });
            for (name, input, encrypt) in [
                ("vigenere_encrypt", &text, true),
                ("vigenere_decrypt", &encrypted, false),
            ] {
                group.bench_with_input(BenchmarkId::new(name, text.len()), input, |b, text| {
                    b.iter(|| {
                        encryption::vigenere_cipher(black_box(text), black_box("LeMoN"), encrypt)
                    });
                });
            }
        }
        group.finish();
    }
}

fn byte_ciphers(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes");
    for len in [0, 64, 4096, 65536] {
        let input: Vec<u8> = (0..=255).cycle().take(len).collect();
        group.throughput(Throughput::Bytes(len as u64));
        for key_len in [1, 16, 256] {
            let key: Vec<u8> = (0..=255).take(key_len).collect();
            let parameter = format!("{len}/key_{key_len}");
            group.bench_with_input(BenchmarkId::new("xor", &parameter), &input, |b, input| {
                b.iter(|| encryption::xor_encrypt(black_box(input), black_box(&key)));
            });
            group.bench_with_input(BenchmarkId::new("rc4", &parameter), &input, |b, input| {
                b.iter(|| encryption::rc4(black_box(input), black_box(&key)));
            });
        }
    }
    group.finish();
}

fn blowfish_blocks(c: &mut Criterion) {
    let mut group = c.benchmark_group("blowfish");
    let block = [0x00, 0x01, 0x7f, 0x80, 0xfe, 0xff, 0xaa, 0x55];
    group.throughput(Throughput::Bytes(block.len() as u64));

    for key_len in [4, 16, 56] {
        let key: Vec<u8> = (0x80..).take(key_len).collect();
        let encrypted = encryption::blowfish_encrypt(&block, &key);
        group.bench_with_input(BenchmarkId::new("encrypt", key_len), &block, |b, block| {
            b.iter(|| encryption::blowfish_encrypt(black_box(block), black_box(&key)));
        });
        group.bench_with_input(
            BenchmarkId::new("decrypt", key_len),
            &encrypted,
            |b, block| {
                b.iter(|| encryption::blowfish_decrypt(black_box(block), black_box(&key)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, text_ciphers, byte_ciphers, blowfish_blocks);
criterion_main!(benches);
