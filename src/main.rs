use rusty_dtw::*;

// TODO: write function to process a list of connectomes
//       create data structure to hold the connectome, so that we can implement the display trait
//       write a configuration object that holds the window and the mode
//       PyO3 bindings

fn main() {
    let connectome = construct_random_connectome(100);
    println!("{:?}", dtw_connectome(&connectome));
}


