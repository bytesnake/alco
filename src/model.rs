use crate::params::ParamSamples;

pub fn estimate_stepsize(results: &[(usize, u64)], min_change: u64) -> usize {
    1
}

pub fn fit_greedy_additive(results: Vec<(ParamSamples, u64)>, beam_size: usize, max_interactions: usize) -> Model {
    Model
}

#[derive(Debug)]
pub struct Model;
