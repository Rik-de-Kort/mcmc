use crate::quality_of_life::{option_to_vec, vec_to_option};
use ndarray_rand::rand::{thread_rng, Rng};

pub struct GibbsChain<D> {
    pub x: Vec<f64>,
    pub pd: D,
}

/// Proposal distribution for Gibbs sampling
/// We use the option type to indicate absence of values
pub trait GibbsProposal {
    // Sample remaining values conditional on x
    fn sample<R: Rng>(&self, x: &[Option<f64>], rng: &mut R) -> Vec<f64>;
    // Conditional density function, p(x | y)
    // Possible fall through if x and y are not complimentary
    // Todo: figure out some way to have an assert in here
    fn pdf(&self, x: &[f64]) -> f64;
}

impl<D: GibbsProposal> Iterator for GibbsChain<D> {
    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = thread_rng();
        let mut result: Vec<Option<f64>> = vec_to_option(&self.x);
        for i in 0..result.len() {
            result[i] = None;
            result = vec_to_option(&self.pd.sample(&result, &mut rng));
        }
        self.x = option_to_vec(result);
        Some(self.x.clone())
    }
}
