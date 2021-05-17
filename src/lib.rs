use rand::{distributions::Standard, Rng};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn dtw(s: &Vec<f32>, t: &Vec<f32>, w: &mut i32, distance: Box<dyn Fn(&f32, &f32) -> f32>) -> f32 {
    let n = s.len() + 1;
    let m = t.len() + 1;
    let mut dtw = vec![vec![f32::MAX; m]; n];
    dtw[0][0] = 0.;

    *w = i32::max(*w, i32::abs((n - m) as i32));
    for si in 1..n {
        let lower_bound = i32::max(1, si as i32 - *w);
        let upper_bound = i32::min(m as i32, si as i32 + *w);
        for ti in lower_bound as usize..upper_bound as usize {
            let cost = distance(&s[si - 1], &t[ti - 1]);
            dtw[si][ti] = cost
                + f32::min(
                    f32::min(dtw[si - 1][ti], dtw[si][ti - 1]),
                    dtw[si - 1][ti - 1],
                );
        }
    }
    f32::sqrt(dtw[s.len()][t.len()])
}

pub fn dtw_connectome(
    connectome: &Vec<Vec<f32>>,
    w: &mut i32,
    distance: Box<dyn Fn(&f32, &f32) -> f32>
) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    for i in 0..connectome.len() {
        for j in 0..i + 1 {
            result.push(dtw(
                &connectome[0..connectome.len()][i],
                &connectome[0..connectome.len()][j],
                &mut 50,
                distance
            ));
        }
    }
    result
}

pub fn dtw_connectomes(
    connectomes: Vec<Vec<Vec<f32>>>,
    n_jobs: u8,
    mut w: i32,
    distance: Box<dyn Fn(&f32, &f32) -> f32>,
) -> Vec<Vec<f32>> {
    let clen = connectomes.len();
    let chunk = ((clen as f32 / n_jobs as f32) as f32).ceil() as usize;
    let mut result = vec![];
    let mut threads = Vec::new();
    let arc = Arc::new(Mutex::new(connectomes));

    for lb in (chunk..clen + chunk).step_by(chunk as usize) {
        let clone = Arc::clone(&arc);
        let t = thread::spawn(move || -> Vec<Vec<f32>> {
            let mut chunk_result = vec![];
            for idx in lb - chunk..usize::min(lb, clen) {
                chunk_result.push(dtw_connectome(&clone.lock().unwrap()[idx], &mut w, distance));
            }
            chunk_result
        });
        threads.push(t);
    }

    for t in threads {
        result.extend(t.join().unwrap());
    }

    // let x = result.lock().unwrap().to_vec(); x
    result
}


pub fn construct_random_connectome(dim: usize) -> Vec<Vec<f32>> {
    let mut connectome: Vec<Vec<f32>> = vec![];
    for _ in 0..dim {
        let values: Vec<f32> = rand::thread_rng().sample_iter(Standard).take(dim).collect();
        connectome.push(values);
    }
    connectome
}

pub struct Config {
    pub mode: String,
    pub window: i32,
    pub n_jobs: u8,
    pub vectorize: bool,
}
