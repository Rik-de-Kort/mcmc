use rand::Rng;
use rand::distributions::{Distribution};
use ::rand_distr::Normal;


fn next<R: Rng>(x: f64, pi: fn(f64) -> f64, proposal: Normal<f64>, rng: &mut R) -> f64 {
    let candidate = x + proposal.sample(rng);

    let alpha = (pi(candidate) / pi(x)).min(1.0);
    let u: f64 = rng.gen();  // Draws uniform [0, 1)
    if u <= alpha {
        return candidate;
    } else {
        return x;
    }
}


pub fn metropolis<R: Rng>(pi: fn(f64) -> f64, proposal: Normal<f64>, rng: &mut R) -> Vec<f64> {
    let local_next = |x: f64, rng: &mut R| { next(x, pi, proposal, rng) };

    // Execute warmup
    let n_warmup = 1e5 as usize;
    let mut x = 10.0;
    for _ in 1..n_warmup{
        x = local_next(x, rng);
    }

    // Start running the simulation
    let n = 1e6 as usize;
    let mut result = Vec::with_capacity(n); 
    result.push(x);

    for i in 1..n {
        let next_val = local_next(result[i-1], rng);
        result.push(next_val);
    }
    return result;
}

