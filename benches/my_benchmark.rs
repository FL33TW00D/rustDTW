use criterion::{criterion_group, criterion_main, Criterion};
use rusty_dtw::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..20{
        connectomes.push(construct_random_connectome(20));
    }
    c.bench_function("dtw_connectome_list", |b| b.iter(|| dtw_connectomes(&connectomes)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
