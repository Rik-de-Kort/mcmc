use ::rand_distr::Normal;
use std::error::Error;

use mcmc::output;
use mcmc::metropolis::metropolis;
use mcmc::quality_of_life::*;


fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let proposal = Normal::new(0.0, 1.0).unwrap();

    // We are looking for a standard normal distribution
    // exp( -x ^ 2 ) is the distribution propertion
    let pi = |x: f32| -> f32 { exp(-x.powi(2)) };
    // let pi = |x: f32| -> f32 { exp(-(55.0 *(x - 2.0)).powi(2)) };

    let result = metropolis(pi, &proposal, &mut rng);

    let (bins, hist) = output::get_hist(result, 500);

    output::write_vec_to_csv(bins, hist)
}


