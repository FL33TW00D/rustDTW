use std::{error::Error};
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;

use numpy::{IntoPyArray, PyArray1, PyArrayDyn, PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};


//TODO:
//Implement a new version of dtw_connectome that takes the vectorize argument and conditionally returns an Array1 or an Array2
//Determine if its possible to use TQDM accurately, since the process will take a while with large matricies
//Add informative error messages, error catching
//Write comprehensive tests covering corner cases like negative numbers etc.
//Write normalizing functions as per Reginas paper

#[pymodule]
fn rust_dtw(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "dtw")]
    fn dtw_py(
        s: PyReadonlyArray1<'_, f64>,
        t: PyReadonlyArray1<'_, f64>,
        window: i32,
        distance_mode: String
    ) -> f64{ 
        dtw(s.as_array().view(), t.as_array().view(), &window, select_distance(&distance_mode).unwrap(), &distance_mode)
    }

    pub fn dtw(s: ArrayView1<f64>, t: ArrayView1<f64>, window: &i32, distance_fn: fn(&f64, &f64) -> f64, distance_mode: &String) -> f64 {
        let m = s.len() + 1;
        let n = t.len() + 1;
        let mut dtw = Array::from_elem((m, n), f64::MAX);
    
        dtw[[0,0]] = 0.;
    
        let max_window = i32::max(*window, i32::abs((n - m) as i32));
        for si in 1..n {
            let lower_bound = i32::max(1, si as i32 - max_window);
            let upper_bound = i32::min(m as i32, si as i32 + max_window);
            for ti in lower_bound as usize..upper_bound as usize {
                let cost = distance_fn(&s[si - 1], &t[ti - 1]);
                dtw[[si, ti]] = cost
                    + f64::min(
                        f64::min(dtw[[si-1, ti]], dtw[[si, ti-1]]),
                        dtw[[si-1, ti-1]],
                    );
            }
        }
        if distance_mode.eq("euclidean"){
            f64::sqrt(dtw[[n-1, m-1]])
        }else{
            dtw[[n-1, m-1]]
        }
    }

    #[pyfn(m, "dtw_connectome")]
    fn dtw_connnectome_py<'py>(
        py: Python<'py>,
        connectome: PyReadonlyArray2<'_, f64>,
        window: i32,
        distance_mode: String
    )-> PyResult<&'py PyArray1<f64>>{
        Ok(dtw_connectome(connectome.as_array().view(), &window,select_distance(&distance_mode).unwrap(), &distance_mode).into_pyarray(py))
    }

    pub fn dtw_connectome(
        connectome: ArrayView2<f64>,
        window: &i32,
        distance_fn: fn(&f64, &f64) -> f64,
        distance_mode: &String
    ) -> Vec<f64> {
        let mut result: Vec<f64> = vec![];
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

    #[pyfn(m, "dtw_connectomes")]
    fn dtw_connnectomes_py<'py>(
        py: Python<'py>,
        connectomes: PyReadonlyArray3<'_, f64>,
        window: i32,
        vectorize: bool,
        distance_mode: String
    )-> &'py PyArrayDyn<f64> {
        dtw_connectomes(connectomes.as_array().to_owned(), &window, vectorize, select_distance(&distance_mode).unwrap(), &distance_mode).into_pyarray(py)
    }
    
    pub fn dtw_connectomes(
        connectomes: Array3<f64>,
        window: &i32,
        vectorize: bool,
        distance_fn: fn(&f64, &f64) -> f64,
        distance_mode: &String
    ) -> ArrayD<f64>{
        let result: Vec<f64> = connectomes.axis_iter(Axis(0))
        .into_par_iter()
        .map(|connectome| dtw_connectome(connectome, window, distance_fn, distance_mode)).flatten().collect();

        if vectorize {
            //TODO: VALIDATE THAT THIS ACTUALLY WORKS SINCE THE ABOVE IS FLATTENED.
            Array2::from_shape_vec((connectomes.shape()[1], connectomes.shape()[1]), result).unwrap().into_dyn()
        }else{
            //convert to each vector (lower triangular) into symmetric matrix
            // Fill Array2 from zeros and then fill lower triangular and + A.T          
            //TODO: FIX BELOW BECAUSE ITS DEAD DEAD WRONG  

            //Now I can extract each slice from the vector, based on the input dim length,
            //convert that into a 2d matrix, or just straight away fill the 3d matrix, probably the best idea
            

            //Now I have the method that converts a vector from the
            Array3::from_shape_vec((connectomes.shape()[0], connectomes.shape()[1], connectomes.shape()[1]), result).unwrap().into_dyn()
            
        }
        
    }

    pub fn ind2tril(ind: f32) -> (f32, f32){
        let i = f32::floor((f32::sqrt(1.0+8.0*ind)-1.0)/2.0);
        let j = ind - i*(i+1.0)/2.0;
        (i, j)
    }


    pub fn vec_to_sym_mat(vec: Array1<f64>, dim: usize) -> Array2<f64>{
        let mut full: Array2<f64> = Array2::zeros((dim, dim));
        for k in 0..vec.len(){
            let (i, j) = ind2tril(k as f32);
            full[[i as usize, j as usize]] = vec[k];
        }
        full.clone() + full.t()
    }

    pub fn select_distance(mode: &str) -> Result<fn(&f64, &f64) -> f64, Box<dyn Error>>{
        match mode {
            "manhattan" => Ok(|a, b| f64::abs(a - b)),
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
//     pub distance_fn: fn(&f64, &f64) -> f64
// }




// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn dtw_test(){
//         let s1:Array1<f64>  = array![0., 0., 1., 2., 1., 0., 1., 0., 0.];
//         let s2:Array1<f64>  = array![0., 1., 2., 0., 0., 0., 0., 0., 0.];

//         let result = dtw(s1.view(), s2.view(), &50, |a, b| (a - b) * (a - b), &String::from("euclidean"));
//         assert_eq!(result, 1.4142135623730951)
//     }
// }