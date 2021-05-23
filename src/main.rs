use rusty_dtw::*;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
// TODO:
//       PyO3 bindings
//      Write method to return as a symmetric connectome

fn main() {
    let distance_mode = String::from("euclidean");
    let config = Config {
        window: 50,
        vectorize: true,
        distance_fn: select_distance(&distance_mode).unwrap(),
        distance_mode
    };

    let connectomes = Array3::random((1, 10, 10), Uniform::new(1., 10.0));
    let result = dtw_connectomes(connectomes, &config.window, config.distance_fn, &config.distance_mode);

    for vec in result.iter() {
        println!("{:?}", vec);
    }
}
