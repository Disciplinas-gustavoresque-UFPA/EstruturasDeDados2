use rand::Rng;

/// Gera um vetor de tamanho `size` com números inteiros aleatórios.
/// Simula o Caso Médio para a maioria dos algoritmos de ordenação.
pub fn generate_random_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..1_000_000)).collect()
}

/// Gera um vetor ordenado de forma crescente.
/// Testa o Melhor Caso (ex: Insertion Sort) ou Pior Caso (ex: QuickSort com pivô no fim).
pub fn generate_sorted_vector(size: usize) -> Vec<i32> {
    (0..size as i32).collect()
}

/// Gera um vetor ordenado de forma decrescente.
/// Simula o Pior Caso estrutural para a maioria das rotinas de particionamento.
pub fn generate_reversed_vector(size: usize) -> Vec<i32> {
    (0..size as i32).rev().collect()
}
