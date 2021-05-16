use rand::random;
use rand::{distributions::Uniform, Rng, distributions::Standard};
use rusty_dtw::dtw_connectome;
fn main() {
    let range = Uniform::from(0..123123);
    let mut connectome: Vec<Vec<f32>> = vec![];
    let DIM = 100;
    for i in 0..DIM {
        let values: Vec<f32> = rand::thread_rng().sample_iter(Standard).take(DIM).collect();
        connectome.push(values);
    }
    
    let result = dtw_connectome(&connectome);
    println!("{:?}", result);
    println!("Produced dimension: {:?}", result.len());
}
