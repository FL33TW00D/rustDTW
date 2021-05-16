use rusty_dtw::*;

// TODO: write function to process a list of connectomes
//       create data structure to hold the connectome, so that we can implement the display trait
//       write a configuration object that holds the window, the mode, to vectorize or not, and the number of jobs.
//       PyO3 bindings

fn main() {
    let mut connectomes: Vec<Vec<Vec<f32>>> = vec![];
    for _ in 0..5{
        connectomes.push(construct_random_connectome(20));
    }

    let result = dtw_connectomes(&connectomes, 4);

    for vec in result.iter(){
        println!("{:?}", vec);
    }

}


