extern crate ndarray;
use ndarray_rand::rand::distributions::Distribution;
use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::Normal;

use std::error::Error;

use mcmc::point_estimate;
use mcmc::quality_of_life::*;

// Defining the proposal distribution
struct Proposal {
    norm: Normal<f64>,
}

use mcmc::gibbs::{GibbsChain, GibbsProposal};
impl GibbsProposal for Proposal {
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

// use mcmc::metropolis::{MetroChain, MetroProposal};
// impl MetroProposal for Proposal {
//     fn sample<R: Rng>(&self, p: &[f64], rng: &mut R) -> Vec<f64> {
//         vec![p[0] + self.norm.sample(rng), p[1] + self.norm.sample(rng)]
//     }
// 
//     fn pdf(&self, p: &[f64], _q: &[f64]) -> f64 {
//         exp(-p[0].powi(2)) * exp(-p[1].powi(2))
//     }
// 
//     fn pi(&self, p: &[f64]) -> f64 {
//         exp(-p[0].powi(2)) * exp(-p[1].powi(2))
//     }
// }


fn f0(x: Vec<f64>) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}

fn main() -> Result<(), Box<dyn Error>> {
    let proposal = Proposal {
        norm: Normal::new(0.0, 1.0).unwrap(),
    };
    let c = GibbsChain {
        x: vec![0.0, 0.0],
        pd: proposal,
    };

    // let c = MetroChain {
    //     x: vec![0.0, 0.0],
    //     pd: proposal,
    // };
    let result = point_estimate(c, f0);

    println!("{}", result);
    Ok(())
}
