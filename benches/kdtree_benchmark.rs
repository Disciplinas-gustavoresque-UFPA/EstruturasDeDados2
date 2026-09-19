use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use estruturas_de_dados_2::kd_tree::{KdTree, Point};

fn generate_random_point_3d() -> Point {
    let mut rng = rand::thread_rng();
    Point {
        // Simulando coordenadas X, Y, Z de uma malha geofísica
        coordinates: vec![
            rng.gen_range(0.0..1000.0), 
            rng.gen_range(0.0..1000.0), 
            rng.gen_range(0.0..1000.0)
        ],
        physics_loss: None,
    }
}

fn bench_nearest_neighbor(c: &mut Criterion) {
    let mut tree = KdTree::new(3); // Espaço 3D
    
    // Popula a árvore com 10.000 pontos (fase de setup, não é medida)
    for _ in 0..10_000 {
        tree.insert(generate_random_point_3d());
    }

    let target = generate_random_point_3d();

    // O Criterion vai medir APENAS o bloco interno do iter()
    c.bench_function("kdtree_nearest_10k_3D", |b| {
        b.iter(|| {
            // black_box impede que o compilador do Rust otimize e anule a função
            tree.nearest(black_box(&target))
        })
    });
}

// Configura os macros do Criterion para rodar a função
criterion_group!(benches, bench_nearest_neighbor);
criterion_main!(benches);