use ::rand_distr::Normal;
use std::error::Error;

use mcmc::output;
use mcmc::metropolis::metropolis;


fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let proposal = Normal::new(0.0, 1.0).unwrap();

    // We are looking for a standard normal distribution
    // exp( -x ^ 2 ) is the distribution propertion
    let pi = |x: f64| -> f64 { (-x.powi(2)).exp() };

    let result = metropolis(pi, &proposal, &mut rng);

    output::write_vec_to_csv(result)
}


