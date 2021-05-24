use std::{error::Error};
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;

use numpy::{IntoPyArray, PyArray1,PyArray2, PyArray3, PyArrayDyn, PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArrayDyn, c64};
use pyo3::{IntoPy, prelude::{pymodule, PyModule, PyResult, Python}};


//TODO:
//Implement a new version of dtw_connectome that takes the vectorize argument and conditionally returns an Array1 or an Array2
//Implement versions of all the methods that return ndarray objects so they can be easily transformed back into numpy arrays for Python users pleasure
//Determine if its possible to use TQDM accurately, since the process will take a while with large matricies
//Add informative error messages, error catching
//Write comprehensive tests covering corner cases like negative numbers etc.
//Work out if any other distance metrics are valid for DTW (minkowski? chebyshev?)

#[pymodule]
fn rust_dtw(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "dtw")]
    fn dtw_py(
        s: PyReadonlyArray1<'_, f32>,
        t: PyReadonlyArray1<'_, f32>,
        window: i32,
        distance_mode: String
    ) -> f32{ 
        dtw(s.as_array().view(), t.as_array().view(), &window, select_distance(&distance_mode).unwrap(), &distance_mode)
    }

    pub fn dtw(s: ArrayView1<f32>, t: ArrayView1<f32>, window: &i32, distance_fn: fn(&f32, &f32) -> f32, distance_mode: &String) -> f32 {
        let m = s.len() + 1;
        let n = t.len() + 1;
        let mut dtw = Array::from_elem((m, n), f32::MAX);
    
        dtw[[0,0]] = 0.;
    
        let max_window = i32::max(*window, i32::abs((n - m) as i32));
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

    #[pyfn(m, "dtw_connectome")]
    fn dtw_connnectome_py<'py>(
        py: Python<'py>,
        connectome: PyReadonlyArray2<'_, f32>,
        window: i32,
        vectorize: bool,
        distance_mode: String
    )-> PyResult<&'py PyArray1<f32>>{
        let result = dtw_connectome(connectome.as_array().view(), &window, select_distance(&distance_mode).unwrap(), &distance_mode);
        Ok(result.into_pyarray(py))
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

    // #[pyfn(m, "dtw_connectomes")]
    // fn dtw_connnectomes_py<'py>(
    //     py: Python<'py>,
    //     connectomes: PyArray3<f32>,
    //     window: i32,
    //     vectorize: bool,
    //     distance_mode: String
    // )-> PyResult<&'py PyArray2<f32>>{
    //     let result = dtw_connectomes(connectomes.as_array().to_owned(), &window, select_distance(&distance_mode).unwrap(), &distance_mode);
    //     Ok(result.into_pyarray(py))
    // }
    
    pub fn dtw_connectomes(
        connectomes: Array3<f32>,
        window: &i32,
        distance_fn: fn(&f32, &f32) -> f32,
        distance_mode: &String
    ) -> Vec<Vec<f32>> {
        connectomes.axis_iter(Axis(0))
            .into_par_iter()
            .map(|connectome| dtw_connectome(connectome, window, distance_fn, distance_mode))
            .collect()
    }

    pub fn select_distance(mode: &str) -> Result<fn(&f32, &f32) -> f32, Box<dyn Error>>{
        match mode {
            "manhattan" => Ok(|a, b| f32::abs(a - b)),
            "euclidean" => Ok(|a, b| (a - b) * (a - b)),
            _ => Err("Please provide a valid distance metric.".into()),
        }
    }

    Ok(())
}





// pub struct Config {
//     pub window: i32,
//     pub vectorize: bool,
//     pub distance_mode: String,
//     pub distance_fn: fn(&f32, &f32) -> f32
// }




// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn dtw_test(){
//         let s1:Array1<f32>  = array![0., 0., 1., 2., 1., 0., 1., 0., 0.];
//         let s2:Array1<f32>  = array![0., 1., 2., 0., 0., 0., 0., 0., 0.];

//         let result = dtw(s1.view(), s2.view(), &50, |a, b| (a - b) * (a - b), &String::from("euclidean"));
//         assert_eq!(result, 1.4142135623730951)
//     }
// }