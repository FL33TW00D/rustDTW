use rusty_dtw::*;

// TODO:
//       create data structure to hold the connectome, so that we can implement the display trait
//       Bring across configuration object from branch
//       Try rayon threading
//       PyO3 bindings

fn main() {
    let config = Config {
        mode: String::from("euclidean"),
        window: 100,
        vectorize: true,
    };

    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..10 {
        connectomes.push(construct_random_connectome(20));
    }

    let distance = select_distance(&config.mode).unwrap();
    let result = dtw_connectomes(connectomes, &config.window, distance);

    for vec in result.iter() {
        println!("{:?}", vec);
    }
}

