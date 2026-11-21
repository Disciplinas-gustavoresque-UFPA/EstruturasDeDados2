//! Casamento exato de cadeias (aula 7): força bruta, BMHS e Shift-And.
//!
//! **Versão sem bibliotecas auxiliares.** Os algoritmos usam só laços,
//! indexação, aritmética e vetores de tamanho fixo (`[T; N]`). Não usam
//! `Vec`, `.len()`, `.push()`, `Option`, iteradores, nem nenhuma função de
//! busca ou ordenação pronta, e não há pacotes externos.
//!
//! Por isso as funções recebem os tamanhos (`n` e `m`) e um vetor de saída
//! (`out`) com sua capacidade (`cap`) como parâmetros.
//!
//! Resposta das funções de busca (um `isize`):
//!
//! - **−1**: entrada inválida (padrão vazio; no Shift-And, padrão com mais de
//!   64 bytes);
//! - **0 ou mais**: o número total de ocorrências. As posições (em bytes) das
//!   primeiras `cap` ocorrências ficam em `out[0..]`.
//!
//! A parte de medição de tempo (`benchmark.rs`) é a única que usa a `std`
//! (`Instant`, `println!` e `vec!`), porque não existe outro modo de medir
//! tempo nem de reservar um texto de 10 milhões de bytes.
//!
//! - `brute_force.rs`: força bruta, usada como referência;
//! - `bmhs.rs`: Boyer-Moore-Horspool-Sunday;
//! - `shift_and.rs`: Shift-And, com paralelismo de bit;
//! - `text_gen.rs`: gerador de texto "Lorem Ipsum" com semente fixa;
//! - `benchmark.rs`: medição de tempo para textos cada vez maiores.

mod benchmark;
mod bmhs;
mod brute_force;
mod shift_and;
mod text_gen;

pub use benchmark::run_benchmark;
pub use bmhs::bmhs;
pub use brute_force::brute_force;
pub use shift_and::shift_and;
pub use text_gen::fill_lorem;

#[cfg(test)]
mod tests {
    use super::{bmhs, brute_force, fill_lorem, shift_and};

    /// Confere se os três algoritmos acham exatamente as posições esperadas.
    fn check_all(text: &[u8], pattern: &[u8], expected: &[usize]) {
        let n = text.len();
        let m = pattern.len();
        let mut out = [0usize; 64];

        let count = brute_force(text, n, pattern, m, &mut out, 64);
        assert_eq!(count, expected.len() as isize);
        let mut k = 0;
        while k < expected.len() {
            assert_eq!(out[k], expected[k]);
            k += 1;
        }

        let count = bmhs(text, n, pattern, m, &mut out, 64);
        assert_eq!(count, expected.len() as isize);
        let mut k = 0;
        while k < expected.len() {
            assert_eq!(out[k], expected[k]);
            k += 1;
        }

        let count = shift_and(text, n, pattern, m, &mut out, 64);
        assert_eq!(count, expected.len() as isize);
        let mut k = 0;
        while k < expected.len() {
            assert_eq!(out[k], expected[k]);
            k += 1;
        }
    }

    #[test]
    fn test_slide_examples() {
        // Exercício 1 (BM): MOORE em BOYERMOORE
        check_all(b"BOYERMOORE", b"MOORE", &[5]);
        // Exercício 2b (Shift-And): MOORE em MOORMOORE
        check_all(b"MOORMOORE", b"MOORE", &[4]);
        // Exemplo do Shift-And: teste em "os testes testam"
        check_all(b"os testes testam", b"teste", &[3]);
        // Exemplo do BMH: cade em hbadecaedcade
        check_all(b"hbadecaedcade", b"cade", &[9]);
    }

    #[test]
    fn test_overlapping_occurrences() {
        check_all(b"aaaa", b"aa", &[0, 1, 2]);
    }

    #[test]
    fn test_no_occurrence() {
        check_all(b"abcdef", b"xyz", &[]);
    }

    #[test]
    fn test_pattern_equal_to_text() {
        check_all(b"abc", b"abc", &[0]);
    }

    #[test]
    fn test_pattern_longer_than_text() {
        check_all(b"ab", b"abc", &[]);
    }

    #[test]
    fn test_empty_text() {
        check_all(b"", b"a", &[]);
    }

    #[test]
    fn test_empty_pattern_is_invalid() {
        let mut out = [0usize; 4];
        assert_eq!(brute_force(b"abc", 3, b"", 0, &mut out, 4), -1);
        assert_eq!(bmhs(b"abc", 3, b"", 0, &mut out, 4), -1);
        assert_eq!(shift_and(b"abc", 3, b"", 0, &mut out, 4), -1);
    }

    #[test]
    fn test_utf8_positions_are_in_bytes() {
        // "á" ocupa 2 bytes (C3 A1): x(0) á(1-2) a(3) á(4-5) a(6)
        check_all(b"x\xc3\xa1a\xc3\xa1a", b"\xc3\xa1a", &[1, 4]);
    }

    #[test]
    fn test_count_beyond_capacity() {
        // Com cap = 2, só 2 posições são guardadas, mas a contagem é total.
        let mut out = [0usize; 2];
        assert_eq!(brute_force(b"aaaa", 4, b"a", 1, &mut out, 2), 4);
        assert_eq!(out[0], 0);
        assert_eq!(out[1], 1);
    }

    #[test]
    fn test_same_answers_on_generated_text() {
        let mut text = [0u8; 20_000];
        fill_lorem(&mut text, 20_000, 2026);

        let mut ref_out = [0usize; 4096];
        let mut out = [0usize; 4096];
        let mut start = 0;
        while start < 19_000 {
            let mut len = 1;
            while len <= 12 {
                let pattern = &text[start..start + len];
                let expected = brute_force(&text, 20_000, pattern, len, &mut ref_out, 4096);
                assert!(expected >= 1);

                let a = bmhs(&text, 20_000, pattern, len, &mut out, 4096);
                assert_eq!(a, expected);
                let mut k = 0;
                while k < expected as usize {
                    assert_eq!(out[k], ref_out[k]);
                    k += 1;
                }

                let b = shift_and(&text, 20_000, pattern, len, &mut out, 4096);
                assert_eq!(b, expected);
                let mut k = 0;
                while k < expected as usize {
                    assert_eq!(out[k], ref_out[k]);
                    k += 1;
                }
                len += 1;
            }
            start += 997;
        }
    }
}
