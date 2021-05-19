use rusty_dtw::*;

// TODO:
//       create data structure to hold the connectome, so that we can implement the display trait
//       PyO3 bindings
//       Remove sqrt from end of function conditionally
//      Write method to return as a symmetric connectome

fn main() {

    let distance = Distance {
        mode: String::from("euclidean"),
        distance: select_distance(mode).unwrap()
    };
    
    let config = Config {
        window: 50,
        vectorize: true,
        distance: distance
    };

    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..1 {
        connectomes.push(construct_random_connectome(300));
    }

    let result = dtw_connectomes(connectomes, &config.window, distance.distance);

    for vec in result.iter() {
        println!("{:?}", vec);
    }
}
