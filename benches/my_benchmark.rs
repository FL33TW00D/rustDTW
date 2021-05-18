use criterion::{criterion_group, criterion_main, Criterion};
use rusty_dtw::*;

fn criterion_benchmark(c: &mut Criterion) {
    let config = Config {
        mode: String::from("euclidean"),
        window: 100,
        vectorize: true,
    };

    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..100 {
        connectomes.push(construct_random_connectome(55));
    }
    let distance = select_distance(&config.mode).unwrap();
    c.bench_function("dtw_connectome_list", |b| {
        b.iter(|| dtw_connectomes(connectomes.clone(), &config.window, distance))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
