use rand::{distributions::Standard, Rng};

pub fn dtw(s: &Vec<f32>, t: &Vec<f32>, w: &mut i32, debug: &bool) -> f32 {
    let n = s.len() + 1;
    let m = t.len() + 1;
    let mut dtw = vec![vec![f32::MAX; m]; n];
    dtw[0][0] = 0.;

    *w = i32::max(*w, i32::abs((n - m) as i32));
    for si in 1..n {
        let lower_bound = i32::max(1, si as i32 - *w);
        let upper_bound = i32::min(m as i32, si as i32 + *w);
        for ti in lower_bound as usize..upper_bound as usize {
            let cost = distance(&s[si - 1], &t[ti - 1], "euclidean").unwrap();
            dtw[si][ti] = cost
                + f32::min(
                    f32::min(dtw[si - 1][ti], dtw[si][ti - 1]),
                    dtw[si - 1][ti - 1],
                );
        }
    }

    // if *debug {
    //     for si in 0..n {
    //         for ti in 0..m {
    //             print!(
    //                 "{:7} ",
    //                 (if dtw[si][ti] == f32::MAX {
    //                     String::from("inf")
    //                 } else {
    //                     format!("{:.*}", 2, dtw[si][ti])
    //                 })
    //             );
    //         }
    //         println!("");
    //     }
    // }
    f32::sqrt(dtw[s.len()][t.len()])
}

pub fn dtw_connectome(connectome: &Vec<Vec<f32>>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    for i in 0..connectome.len() {
        //i+1 includes main diagonal, which is typically 0'd anyway but makes it easier when we want to convert from vector -> matrix
        for j in 0..i + 1 {
            result.push(dtw(
                &connectome[0..connectome.len()][i],
                &connectome[0..connectome.len()][j],
                &mut 50,
                &false,
            ));
        }
    }
    result
}

fn distance(a: &f32, b: &f32, mode: &str) -> Result<f32, String> {
    match mode {
        "manhattan" => Ok(f32::abs(a - b)),
        "euclidean" => Ok((a - b) * (a - b)),
        __ => Err(String::from("Please provide a valid distance metric.")),
    }
}

pub fn construct_random_connectome(dim: usize) -> Vec<Vec<f32>> {
    let mut connectome: Vec<Vec<f32>> = vec![];
    for _ in 0..dim {
        let values: Vec<f32> = rand::thread_rng().sample_iter(Standard).take(dim).collect();
        connectome.push(values);
    }
    connectome
}