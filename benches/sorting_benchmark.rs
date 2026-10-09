use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
// Importa a nossa fábrica de dados do módulo utils
use estruturas_de_dados_2::utils::{generate_random_vector, generate_reversed_vector, generate_sorted_vector};

pub fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("Analise de Complexidade - Algoritmos de Ordenacao");
    
    // Tamanho do vetor (n)
    let size = 10_000;

    // Teste 1: Caso Médio (Vetor Aleatório)
    group.bench_function(format!("Baseline (Std Sort) - Aleatorio - n={}", size), |b| {
        b.iter_batched(
            || generate_random_vector(size), // Setup: gera os dados antes de iniciar o cronômetro
            |mut data| {
                data.sort(); // Ação medida
                black_box(data) // Evita que o compilador ignore a operação
            },
            BatchSize::SmallInput,
        )
    });

    // Teste 2: Pior Caso Teórico (Vetor Invertido)
    group.bench_function(format!("Baseline (Std Sort) - Invertido - n={}", size), |b| {
        b.iter_batched(
            || generate_reversed_vector(size),
            |mut data| {
                data.sort();
                black_box(data)
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

// Configura e executa o grupo de benchmarks
criterion_group!(benches, bench_sorting);
criterion_main!(benches);