use rusty_dtw::*;

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

    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..1 {
        connectomes.push(construct_random_connectome(10));
    }

    let result = dtw_connectomes(connectomes, &config.window, config.distance_fn, &config.distance_mode);

    for vec in result.iter() {
        println!("{:?}", vec);
    }
}
