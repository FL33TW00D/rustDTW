use rand::{distributions::Standard, Rng};
use rayon::prelude::*;
use std::{error::Error};

pub fn dtw(s: &Vec<f32>, t: &Vec<f32>, w: &i32, distance_fn: fn(&f32, &f32) -> f32, distance_mode: &String) -> f32 {
    let n = s.len() + 1;
    let m = t.len() + 1;
    let mut dtw = vec![vec![f32::MAX; m]; n];
    dtw[0][0] = 0.;

    let max_window = i32::max(*w, i32::abs((n - m) as i32));
    for si in 1..n {
        let lower_bound = i32::max(1, si as i32 - max_window);
        let upper_bound = i32::min(m as i32, si as i32 + max_window);
        for ti in lower_bound as usize..upper_bound as usize {
            let cost = distance_fn(&s[si - 1], &t[ti - 1]);
            dtw[si][ti] = cost
                + f32::min(
                    f32::min(dtw[si - 1][ti], dtw[si][ti - 1]),
                    dtw[si - 1][ti - 1],
                );
        }
    }
    if distance_mode.eq("euclidean"){
        f32::sqrt(dtw[s.len()][t.len()])
    }else{
        dtw[s.len()][t.len()]
    }
}

pub fn dtw_connectome(
    connectome: &Vec<Vec<f32>>,
    window: &i32,
    distance_fn: fn(&f32, &f32) -> f32,
    distance_mode: &String
) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    for i in 0..connectome.len() {
        for j in 0..i + 1 {
            result.push(dtw(
                &connectome[0..connectome.len()][i],
                &connectome[0..connectome.len()][j],
                window,
                distance_fn,
                distance_mode,
            ));
        }
    }
    result
}

pub fn dtw_connectomes(
    connectomes: Vec<Vec<Vec<f32>>>,
    window: &i32,
    distance_fn: fn(&f32, &f32) -> f32,
    distance_mode: &String
) -> Vec<Vec<f32>> {
    connectomes
        .par_iter()
        .map(|connectome| dtw_connectome(connectome, window, distance_fn, distance_mode))
        .collect()
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
    pub window: i32,
    pub vectorize: bool,
    pub distance_mode: String,
    pub distance_fn: fn(&f32, &f32) -> f32
}


pub fn select_distance(mode: &str) -> Result<fn(&f32, &f32) -> f32, Box<dyn Error>>{
    match mode {
        "manhattan" => Ok(|a, b| f32::abs(a - b)),
        "euclidean" => Ok(|a, b| (a - b) * (a - b)),
        _ => Err("Please provide a valid distance metric.".into()),
    }
}
