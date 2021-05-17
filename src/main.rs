use rusty_dtw::*;

// TODO: write function to process a list of connectomes
//       create data structure to hold the connectome, so that we can implement the display trait
//       write a configuration object that holds the window, the mode, to vectorize or not, and the number of jobs.
//       PyO3 bindings

fn main() {
    let config = Config {
        mode: String::from("euclidean"),
        window: 100,
        n_jobs: 8,
        vectorize: true,
    };

    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..10 {
        connectomes.push(construct_random_connectome(20));
    }

    let distance = select_distance(&config.mode);
    let result = dtw_connectomes(connectomes, config.n_jobs, config.window, distance);

    for vec in result.iter() {
        println!("{:?}", vec);
    }
}

fn select_distance(mode:&str) -> Box<dyn Fn(&f32, &f32) -> f32> {
    match mode {
        "manhattan" => Box::new(|a,b| f32::abs(a - b)),
        "euclidean" => Box::new(|a, b| (a - b) * (a - b))
    }
}
