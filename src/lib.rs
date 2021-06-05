use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use std::error::Error;

use numpy::{
    IntoPyArray, PyArray1, PyArrayDyn, PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3,
};
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

//TODO:
//Determine if its possible to use TQDM accurately, since the process will take a while with large matricies
//Add informative error messages, error catching
//Write normalizing functions as per Reginas paper

#[pymodule]
fn rust_dtw(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "dtw")]
    fn dtw_py(
        s: PyReadonlyArray1<'_, f64>,
        t: PyReadonlyArray1<'_, f64>,
        window: i32,
        distance_mode: String,
    ) -> f64 {
        dtw(
            s.as_array().view(),
            t.as_array().view(),
            &window,
            select_distance(&distance_mode).unwrap(),
            &distance_mode,
        )
    }

    /// Dynamic time warping distance between 2 1D matricies
    ///
    /// # Arguments
    ///
    /// * `s` - A 1D array
    /// * `t` - A 1D array
    /// * `window` - warping window
    /// * `distance_fn` - a function object that computes the distance between 2 points
    /// * `distance_mode` - string repr of above function

    pub fn dtw(
        s: ArrayView1<f64>,
        t: ArrayView1<f64>,
        window: &i32,
        distance_fn: fn(&f64, &f64) -> f64,
        distance_mode: &String,
    ) -> f64 {
        let m = s.len() + 1;
        let n = t.len() + 1;
        let mut dtw = Array::from_elem((m, n), f64::MAX);

        dtw[[0, 0]] = 0.;

        let max_window = i32::max(*window, i32::abs((n - m) as i32));
        for si in 1..n {
            let lower_bound = i32::max(1, si as i32 - max_window);
            let upper_bound = i32::min(m as i32, si as i32 + max_window);
            for ti in lower_bound as usize..upper_bound as usize {
                let cost = distance_fn(&s[si - 1], &t[ti - 1]);
                dtw[[si, ti]] = cost
                    + f64::min(
                        f64::min(dtw[[si - 1, ti]], dtw[[si, ti - 1]]),
                        dtw[[si - 1, ti - 1]],
                    );
            }
        }
        if distance_mode.eq("euclidean") {
            f64::sqrt(dtw[[n - 1, m - 1]])
        } else {
            dtw[[n - 1, m - 1]]
        }
    }

    #[pyfn(m, "dtw_connectome")]
    fn dtw_connnectome_py<'py>(
        py: Python<'py>,
        connectome: PyReadonlyArray2<'_, f64>,
        window: i32,
        distance_mode: String,
    ) -> PyResult<&'py PyArray1<f64>> {
        Ok(dtw_connectome(
            connectome.as_array().view(),
            &window,
            select_distance(&distance_mode).unwrap(),
            &distance_mode,
        )
        .into_pyarray(py))
    }

    /// Dynamic time warping on a 2D matrix representing an fMRI timeseries
    ///
    /// # Arguments
    ///
    /// * `connectome` - An (m x n) matrix (number of time points x number of ROIs)
    /// * `window` - warping window
    /// * `distance_fn` - a function object that computes the distance between 2 points
    /// * `distance_mode` - string repr of above function

    pub fn dtw_connectome(
        connectome: ArrayView2<f64>,
        window: &i32,
        distance_fn: fn(&f64, &f64) -> f64,
        distance_mode: &String,
    ) -> Vec<f64> {
        let mut result: Vec<f64> = vec![];
        for i in 0..connectome.shape()[1] {
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
        distance_mode: String,
    ) -> &'py PyArrayDyn<f64> {
        dtw_connectomes(
            connectomes.as_array().to_owned(),
            &window,
            vectorize,
            select_distance(&distance_mode).unwrap(),
            &distance_mode,
        )
        .into_pyarray(py)
    }

    pub fn dtw_connectomes(
        connectomes: Array3<f64>,
        window: &i32,
        vectorize: bool,
        distance_fn: fn(&f64, &f64) -> f64,
        distance_mode: &String,
    ) -> ArrayD<f64> {
        //Connectomes is a 3D array (n_subjects x n_samples x n_features)
        let (n_subjects, _n_samples, n_features) = connectomes.dim();
        let vec_len = n_features * (n_features + 1) / 2;

        let result: Vec<f64> = connectomes
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|connectome| dtw_connectome(connectome, window, distance_fn, distance_mode))
            .flatten()
            .collect();

        if vectorize {
            Array2::from_shape_vec((n_subjects, vec_len), result)
                .unwrap()
                .into_dyn()
        } else {
            let mut sym: Array3<f64> = Array3::zeros((n_subjects, n_features, n_features));
            for (idx, mut segment) in sym.axis_iter_mut(Axis(0)).enumerate() {
                segment += &vec_to_sym_mat(
                    &result[vec_len * idx..vec_len * (idx + 1)].to_vec(),
                    n_features,
                );
            }
            sym.into_dyn()
        }
    }

    pub fn ind2tril(ind: f32) -> (f32, f32) {
        let i = f32::floor((f32::sqrt(1.0 + 8.0 * ind) - 1.0) / 2.0);
        let j = ind - i * (i + 1.0) / 2.0;
        (i, j)
    }

    pub fn vec_to_sym_mat(vec: &Vec<f64>, dim: usize) -> Array2<f64> {
        let mut full: Array2<f64> = Array2::zeros((dim, dim));
        for k in 0..vec.len() {
            let (i_l, j_l) = ind2tril(k as f32);
            full[[i_l as usize, j_l as usize]] = vec[k];
        }
        //cloning is slow, will improve in future
        full += &full.clone().t();
        full
    }

    pub fn select_distance(mode: &str) -> Result<fn(&f64, &f64) -> f64, Box<dyn Error>> {
        match mode {
            "manhattan" => Ok(|a, b| f64::abs(a - b)),
            "euclidean" => Ok(|a, b| (a - b) * (a - b)),
            _ => Err("Please provide a valid distance metric.".into()),
        }
    }

    Ok(())
}
