use criterion::{criterion_group, criterion_main, Criterion};
use rusty_dtw::*;

fn criterion_benchmark(c: &mut Criterion) {
    let connectome = construct_random_connectome(100);
    c.bench_function("dtw_connectome", |b| b.iter(|| dtw_connectome(&connectome)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
