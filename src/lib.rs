use rand::{distributions::Standard, Rng};
use std::{error::Error};
use ndarray::parallel::prelude::*;

use ndarray::prelude::*;


pub fn dtw(s: ArrayView1<f32>, t: ArrayView1<f32>, w: &i32, distance_fn: fn(&f32, &f32) -> f32, distance_mode: &String) -> f32 {
    let m = s.len() + 1;
    let n = t.len() + 1;
    let mut dtw = Array::from_elem((m, n), f32::MAX);

    dtw[[0,0]] = 0.;

    let max_window = i32::max(*w, i32::abs((n - m) as i32));
    for si in 1..n {
        let lower_bound = i32::max(1, si as i32 - max_window);
        let upper_bound = i32::min(m as i32, si as i32 + max_window);
        for ti in lower_bound as usize..upper_bound as usize {
            let cost = distance_fn(&s[si - 1], &t[ti - 1]);
            dtw[[si, ti]] = cost
                + f32::min(
                    f32::min(dtw[[si-1, ti]], dtw[[si, ti-1]]),
                    dtw[[si-1, ti-1]],
                );
        }
    }
    if distance_mode.eq("euclidean"){
        f32::sqrt(dtw[[n-1, m-1]])
    }else{
        dtw[[n-1, m-1]]
    }
}

pub fn dtw_connectome(
    connectome: ArrayView2<f32>,
    window: &i32,
    distance_fn: fn(&f32, &f32) -> f32,
    distance_mode: &String
) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    for i in 0..connectome.shape()[0]{ 
        for j in 0..i + 1 {
            result.push(dtw(
                connectome.slice(s![.., i]),
                connectome.slice(s![.., j]),                                
                window,
                distance_fn,
                distance_mode,
            ));
        }
    }
    result
}

pub fn dtw_connectomes(
    connectomes: Array3<f32>,
    window: &i32,
    distance_fn: fn(&f32, &f32) -> f32,
    distance_mode: &String
) -> Vec<Vec<f32>> {
    connectomes.axis_iter(Axis(0))
        .into_iter()
        .map(|connectome| dtw_connectome(connectome, window, distance_fn, distance_mode))
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dtw_test(){
        let s1:Array1<f32>  = array![0., 0., 1., 2., 1., 0., 1., 0., 0.];
        let s2:Array1<f32>  = array![0., 1., 2., 0., 0., 0., 0., 0., 0.];

        let result = dtw(s1.view(), s2.view(), &50, |a, b| (a - b) * (a - b), &String::from("euclidean"));
        assert_eq!(result, 1.4142135623730951)
    }
}