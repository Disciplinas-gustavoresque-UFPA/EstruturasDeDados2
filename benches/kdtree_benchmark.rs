use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use estruturas_de_dados_2::searching::kd_tree::{KdTree, Tree};

/// Gera um ponto aleatório num espaço tridimensional para testes de carga.
fn generate_random_point_3d() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    vec![
        rng.gen_range(0.0..1000.0), 
        rng.gen_range(0.0..1000.0), 
        rng.gen_range(0.0..1000.0)
    ]
}

/// Executa o teste de desempenho para a procura do vizinho mais próximo.
fn bench_nearest_neighbor(c: &mut Criterion) {
    let mut tree: KdTree = Tree::new(3);
    
    // Pré-carrega a árvore com 10.000 pontos aleatórios
    for _ in 0..10_000 {
        tree.insert(generate_random_point_3d());
    }

    let target = generate_random_point_3d();

    // O black_box impede que o compilador otimize e ignore a função
    c.bench_function("kdtree_nearest_10k_3D", |b| {
        b.iter(|| {
            tree.nearest(black_box(&target))
        })
    });
}

criterion_group!(benches, bench_nearest_neighbor);
criterion_main!(benches);