extern crate csv; 

use rand::Rng;
use rand::distributions::{Distribution};
use ::rand_distr::Normal;

use std::error::Error;
use mcmc::output;


// mod output {
//     use std::error::Error;
//     use csv::Writer;
// 
//     pub fn write_vec_to_csv(result: Vec<f64>) -> Result<(), Box<dyn Error>> {
//         let mut wtr = Box::new(Writer::from_path("data.csv")?);
//         wtr.write_record(&["index", "val"])?;
// 
//         for i in 0..result.len()-1 {
//             wtr.write_record(&[i.to_string(), result[i].to_string()])?;
//         }
//         wtr.flush()?;
//         Ok(())
//     }
// }


fn next<R: Rng>(x: f64, pi: fn(f64) -> f64, proposal: Normal<f64>, mut rng: R) -> f64 {
    let candidate = x + proposal.sample(&mut rng);

    let alpha = (pi(candidate) / pi(x)).min(1.0);
    let u: f64 = rng.gen();
    if u <= alpha {
        return candidate;
    } else {
        return x;
    }
}

fn metropolis(pi: fn(f64) -> f64, proposal: Normal<f64>) -> Vec<f64> {
    let rng = rand::thread_rng();
    let local_next = |x: f64| { next(x, pi, proposal, rng) };

    // Execute warmup
    let n_warmup = 10000;
    let mut x = 10.0;
    for _ in 1..n_warmup{
        x = local_next(x);
    }

    // Start running the simulation
    let n = 1_000_000;
    let mut result = Vec::with_capacity(n); 
    result.push(x);

    for i in 1..n {
        let next_val = local_next(result[i-1]);
        result.push(next_val);
    }
    return result;
}



fn main() -> Result<(), Box<dyn Error>> {
    let proposal = Normal::new(0.0, 1.0).unwrap();

    // We are looking for a standard normal distribution
    // exp( -x ^ 2 ) is the distribution propertion
    let pi = |x: f64| -> f64 { (-x.powi(2)).exp() };

    let result = metropolis(pi, proposal);

    output::write_vec_to_csv(result)
}


