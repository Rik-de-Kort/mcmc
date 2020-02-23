extern crate ndarray;
use ndarray_rand::rand_distr::Normal;

use std::error::Error;

use mcmc::metropolis::metropolis;
use mcmc::output;
use mcmc::quality_of_life::*;



fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = ndarray_rand::rand::thread_rng();
    let sampler = build_sampler(Normal::new(0.0, 1.0).unwrap());

    // We are looking for a standard normal distribution
    // exp( -x ^ 2 ) is the distribution propertion
    let pi = |x: &f64| -> f64 { exp(-x.powi(2)) };

    let result = metropolis(pi, &sampler, &mut rng);

    let (bins, hist) = output::get_hist(result, 500);

    output::write_vec_to_csv(bins, hist)
}
