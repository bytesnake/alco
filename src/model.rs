use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;
use crate::params::ParamSamples;

pub fn estimate_stepsize(results: &[(usize, u64)], min_change: u64) -> usize {
    if results.len() < 2 {
        return results[results.len()-1].0 + 1;
    }

    let x = Array1::from_iter(results.iter().map(|x| x.1 as f32));
    let d: Vec<f32> = results.iter().map(|x| {
        let mut n = x.0 as f32 + 1.0;
        let mut out = Vec::with_capacity(results.len());

        for i in 0..results.len() {
            n = n * (x.0 as f32 + 1.0);
            out.push(n);
        }

        out
    }).flatten().collect();
    let d = Array2::from_shape_vec((results.len(), results.len()), d).unwrap();

    dbg!(&results);
    dbg!(&x, &d);


    let beta = d.solve(&x).unwrap();

    dbg!(&beta);

    // perform newton-raphson
    let next_target = results[results.len()-1] + min_change as f32;
    let fnc = |z: f32| -> f32 { next_target -

    1
}

pub fn fit_greedy_additive(results: Vec<(ParamSamples, u64)>, beam_size: usize, max_interactions: usize) -> Model {
    Model
}

#[derive(Debug)]
pub struct Model;
