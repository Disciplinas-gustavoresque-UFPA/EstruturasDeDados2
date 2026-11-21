//! Medição de tempo dos três algoritmos para textos cada vez maiores.
//!
//! Este é o único arquivo que usa a `std`: `Instant` para medir o tempo,
//! `println!` para imprimir e `vec!` para reservar o texto (de até 10
//! milhões de bytes, grande demais para um vetor de tamanho fixo na pilha).
//! Os algoritmos medidos não usam nada disso.
//!
//! Para ver o resultado (em modo `release`, que é o que vale para medir):
//!
//! ```text
//! cargo test --release print_benchmark -- --ignored --nocapture
//! ```
//!
//! A saída é um CSV (`tamanho_bytes,algoritmo,mediana_ns`), pronto para
//! virar o gráfico comparativo em uma planilha.

use super::bmhs::bmhs;
use super::brute_force::brute_force;
use super::shift_and::shift_and;
use super::text_gen::fill_lorem;
use std::time::Instant;

/// Quantos tamanhos de texto são medidos.
const NUM_SIZES: usize = 4;

/// Tamanhos de texto medidos, em bytes.
const SIZES: [usize; NUM_SIZES] = [10_000, 100_000, 1_000_000, 10_000_000];

/// Quantas vezes cada medição é repetida (a mediana é a resposta).
const REPETITIONS: usize = 5;

/// Padrão buscado e o seu tamanho.
const PATTERN: &[u8] = b"consectetur adipiscing";
const PATTERN_LEN: usize = 22;

/// Tipo das três funções de busca.
type Search = fn(&[u8], usize, &[u8], usize, &mut [usize], usize) -> isize;

/// Mediana de um vetor de tempos. Ordena por inserção, à mão.
fn median(values: &mut [u128; REPETITIONS]) -> u128 {
    let mut i = 1;
    while i < REPETITIONS {
        let mut j = i;
        while j > 0 && values[j - 1] > values[j] {
            let tmp = values[j - 1];
            values[j - 1] = values[j];
            values[j] = tmp;
            j -= 1;
        }
        i += 1;
    }
    values[REPETITIONS / 2]
}

/// Tempo mediano, em nanossegundos, de uma busca repetida `REPETITIONS` vezes.
fn time_one(search: Search, text: &[u8], n: usize) -> u128 {
    let mut times = [0u128; REPETITIONS];
    let mut out = [0usize; 64];
    let mut r = 0;
    while r < REPETITIONS {
        let start = Instant::now();
        let result = search(text, n, PATTERN, PATTERN_LEN, &mut out, 64);
        times[r] = start.elapsed().as_nanos();
        if result < 0 {
            println!("erro: entrada inválida");
        }
        r += 1;
    }
    median(&mut times)
}

/// Mede os três algoritmos para cada tamanho de texto e imprime um CSV.
pub fn run_benchmark() {
    println!("tamanho_bytes,algoritmo,mediana_ns");
    let mut s = 0;
    while s < NUM_SIZES {
        let size = SIZES[s];
        let mut text = vec![0u8; size];
        fill_lorem(&mut text, size, 2026);
        println!("{},forca_bruta,{}", size, time_one(brute_force, &text, size));
        println!("{},bmhs,{}", size, time_one(bmhs, &text, size));
        println!("{},shift_and,{}", size, time_one(shift_and, &text, size));
        s += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::run_benchmark;

    #[test]
    #[ignore]
    fn print_benchmark() {
        run_benchmark();
    }
}
