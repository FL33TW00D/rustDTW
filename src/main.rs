use rusty_dtw::*;

// TODO:
//       create data structure to hold the connectome, so that we can implement the display trait
//       PyO3 bindings
//       Remove sqrt from end of function conditionally
//      Write method to return as a symmetric connectome

fn main() {
    let config = Config {
        mode: String::from("minkowski"),
        window: 50,
        vectorize: true,
    };

    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..1 {
        connectomes.push(construct_random_connectome(300));
    }

    let distance = select_distance(&config.mode).unwrap();
    let result = dtw_connectomes(connectomes, &config.window, distance);

    for vec in result.iter() {
        println!("{:?}", vec);
    }
}
