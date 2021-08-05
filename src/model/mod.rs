mod chebyshev;

use ndarray::{Array1, Array2};
use ndarray_linalg::least_squares::LeastSquaresSvd;
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
    if n < 2 {
        return results[n-1].0 + 1;
    }

    /*let targets = Array1::from_iter(results.iter().map(|x| (x.1 as f32).ln()));
    let polys: Vec<f32> = results.iter()
        .map(|x| poly_fnc(x.0 as f32 + 2.0, n))
        .flatten()
        .collect();

    let polys = Array2::from_shape_vec((n, n), polys).unwrap();

    dbg!(&results);
    dbg!(&targets, &polys);

    let beta = polys.least_squares(&targets).unwrap();
    dbg!(&beta);
    let beta = beta.solution;*/

    let sol = chebyshev::Chebyshev::new((results.iter().map(|x| x.0 as f32).collect(), results.iter().map(|x| x.1 as f32).collect()), results.len());

    /*

    dbg!((targets - polys.dot(&beta)).mapv(|x| x*x).sum());

    // perform newton-raphson
    let next_target = ((results[n-1].1 + 500) as f32).ln();
    let fnc = |z: f32| -> f32 { next_target - poly_fnc(z, n).iter().zip(beta.iter()).map(|(a, b)| a*b).sum::<f32>() };
    let d_fnc = |z: f32| -> f32 { - poly_fnc_deriv(z, n).iter().zip(beta.iter()).map(|(a, b)| a*b).sum::<f32>() };

    let mut z = results[n-1].0 as f32;
    for _ in 0..20 {
        z -= fnc(z) / (d_fnc(z) + 1e-5);
    }
    dbg!(&z);
    dbg!(fnc(z));

    println!(" #### DONE \n\n");

    z.ceil() as usize - results[n-1].0 */
    1
}

pub fn fit_greedy_additive(results: Vec<(ParamSamples, u64)>, beam_size: usize, max_interactions: usize) -> Model {
    Model
}

#[derive(Debug)]
pub struct Model;
