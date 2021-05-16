use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::Standard, distributions::Uniform, Rng}; // 0.8.0
use rand::random;
use rusty_dtw::dtw_connectome;

fn criterion_benchmark(c: &mut Criterion) {
    let range = Uniform::from(0..123123);
    let mut connectome: Vec<Vec<f32>> = vec![];
    let DIM = 20;
    for i in 0..DIM {
        let values: Vec<f32> = rand::thread_rng().sample_iter(Standard).take(DIM).collect();
        connectome.push(values);
    }
    
    
    c.bench_function("dtw_connectome", |b| b.iter(|| dtw_connectome(&connectome)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
