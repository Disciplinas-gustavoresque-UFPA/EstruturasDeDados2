use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use estruturas_de_dados_2::searching::kd_tree::{KdTree, Point};

fn generate_random_point_3d() -> Point {
    let mut rng = rand::thread_rng();
    Point {
        coordinates: vec![
            rng.gen_range(0.0..1000.0), 
            rng.gen_range(0.0..1000.0), 
            rng.gen_range(0.0..1000.0)
        ],
    }
}

fn bench_nearest_neighbor(c: &mut Criterion) {
    let mut tree = KdTree::new(3);
    
    for _ in 0..10_000 {
        tree.insert(generate_random_point_3d());
    }

    let target = generate_random_point_3d();

    c.bench_function("kdtree_nearest_10k_3D", |b| {
        b.iter(|| {
            tree.nearest(black_box(&target))
        })
    });
}

criterion_group!(benches, bench_nearest_neighbor);
criterion_main!(benches);