extern crate ndarray;
use ndarray_rand::rand::distributions::Distribution;
use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::Normal;

use std::error::Error;

use mcmc::gibbs::{point_estimate, GibbsChain, ProposalDistribution};
use mcmc::quality_of_life::*;

// Defining the proposal distribution
struct Proposal {
    norm: Normal<f64>,
}

impl ProposalDistribution for Proposal {
    fn sample<R: Rng>(&self, p: &[Option<f64>], rng: &mut R) -> Vec<f64> {
        assert!(p.len() == 2); // Panic if not 2d

        p.iter()
            .map(|item| match item {
                None => self.norm.sample(rng),
                Some(x) => *x,
            })
            .collect()
    }

    fn pdf(&self, p: &[f64]) -> f64 {
        exp(-p[0].powi(2)) * exp(-p[0].powi(2))
    }
}

fn f0(x: Vec<f64>) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn main() -> Result<(), Box<dyn Error>> {
    let proposal = Proposal {
        norm: Normal::new(0.0, 1.0).unwrap(),
    };
    let gc = GibbsChain {
        x: vec![0.0, 0.0],
        pd: proposal,
    };
    let result = point_estimate(gc, f0);

    println!("{}", result);
    Ok(())
}
