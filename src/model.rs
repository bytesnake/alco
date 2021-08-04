use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;
use crate::params::ParamSamples;

/// https://stackoverflow.com/questions/382186/fitting-polynomials-to-data
fn poly_fnc(val: f32, n: usize) -> Vec<f32> {
    let ln_val = val.ln();
    (1..=n).map(|x| (x as f32) * ln_val).collect()
}


fn poly_fnc_deriv(val: f32, n: usize) -> Vec<f32> {
    (1..=n).map(|x| (x as f32) / val).collect()
}

pub fn estimate_stepsize(results: &[(usize, u64)], min_change: u64) -> usize {
    let n = results.len();
    if n < 4 {
        return results[n-1].0 + 1;
    }

    let targets = Array1::from_iter(results.iter().map(|x| (x.1 as f32).ln()));
    let polys: Vec<f32> = results.iter()
        .map(|x| poly_fnc(x.0 as f32 + 2.0, n))
        .flatten()
        .collect();

    let polys = Array2::from_shape_vec((n, n), polys).unwrap();

    dbg!(&results);
    dbg!(&targets, &polys);

    let beta = polys.solve(&targets).unwrap();

    dbg!(&beta);

    // perform newton-raphson
    let next_target = (results[n-1].1 + min_change) as f32;
    let fnc = |z: f32| -> f32 { next_target - poly_fnc(z, n).iter().zip(beta.iter()).map(|(a, b)| a*b).sum::<f32>() };
    let d_fnc = |z: f32| -> f32 { - poly_fnc_deriv(z, n).iter().zip(beta.iter()).map(|(a, b)| a*b).sum::<f32>() };

    panic!("");
    let mut z = results[n-1].0 as f32;
    dbg!(&next_target);
    for _ in 0..50 {
        z -= fnc(z) / (d_fnc(z) + 1e-5);
    }


    1
}

pub fn fit_greedy_additive(results: Vec<(ParamSamples, u64)>, beam_size: usize, max_interactions: usize) -> Model {
    Model
}

#[derive(Debug)]
pub struct Model;
