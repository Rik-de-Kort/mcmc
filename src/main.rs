extern crate ndarray;
use ndarray_rand::rand::distributions::Distribution;
use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::Normal;

use std::error::Error;

use mcmc::gibbs::{gibbs, ProposalDistribution};
use mcmc::output;
use mcmc::quality_of_life::*;

// Defining the proposal distribution
struct Proposal {
    norm: Normal<f64>,
}

impl ProposalDistribution for Proposal {
    fn sample<R: Rng>(&self, p: &Vec<Option<f64>>, rng: &mut R) -> Vec<f64> {
        assert!(p.len() == 2); // Panic if not 2d

        p.iter().map( | item | {
            match item {
                None => self.norm.sample(rng),
                Some(x) => x
            }
        }).collect()

     //    P {
     //        x: p.x + self.norm.sample(rng),
     //        y: p.y + self.norm.sample(rng),
     //    }
    }

    fn pdf(&self, p: &Vec<f64>, _q: &Vec<f64>) -> f64 {
        exp(-p[0].powi(2)) * exp(-p[0].powi(2))
    }
}

fn pi(p: Vec<f64>) -> f64 {
    exp(-p[0].powi(2)) * exp(-p[1].powi(2))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = ndarray_rand::rand::thread_rng();

    let prop = Proposal {
        norm: Normal::new(0.0, 1.0).unwrap(),
    };
    let initial = vec![0.0, 0.0];
    let result = gibbs(initial, pi, prop, &mut rng);

    output::write_vec_to_csv(result)
}
