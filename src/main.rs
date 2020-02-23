extern crate ndarray;
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::rand::Rng;
use ndarray_rand::rand::distributions::Distribution;

use std::error::Error;

use mcmc::metropolis::metropolis;
use mcmc::output;
use mcmc::quality_of_life::*;

use std::ops::Add;
use serde::Serialize;


#[derive(Copy, Clone, Debug)]
#[derive(Serialize)]
struct P { x: f64, y: f64 }
impl Add for P {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        P { x: self.x + other.x, y: self.y + other.y }
    }
}



fn gaussian_2d(p: &P) -> f64 {
    exp(-p.x.powi(2)) * exp(-p.y.powi(2))
}

fn sampler(rng: &mut impl Rng) -> P {
    let dist = Normal::new(0.0, 1.0).unwrap();
    P{x: dist.sample(rng), y: dist.sample(rng)}
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = ndarray_rand::rand::thread_rng();
    let initial = P{x: 0.0, y: 0.0};
    let result = metropolis(initial, gaussian_2d, &sampler, &mut rng);

    // let (bins, hist) = output::get_hist(result, 500);
    // output::write_vec_to_csv(bins, hist)
    // output::write_vec_to_csv((0..result.len()).collect(), result)
    output::write_vec_to_csv(result)
}
