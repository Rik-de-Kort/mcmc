use num;
use num::{Float, FromPrimitive};

use rand::Rng;
use rand::distributions::{Distribution};


fn next<R: Rng, F: Float+FromPrimitive, D: Distribution<F>>(x: F, pi: fn(F) -> F, proposal: &D, rng: &mut R) -> F {
    let candidate = x + proposal.sample(rng);

    let alpha = (pi(candidate) / pi(x)).min(F::one());
    let u = F::from_f64(rng.gen()).unwrap();  // Draws uniform [0, 1)
    if u <= alpha {
        return candidate;
    } else {
        return x;
    }
}


pub fn metropolis<R: Rng, F: Float+FromPrimitive, D: Distribution<F>>(pi: fn(F) -> F, proposal: &D, rng: &mut R) -> Vec<F> {
    let local_next = |x: F, rng: &mut R| { next(x, pi, proposal, rng) };

    // Execute warmup
    let n_warmup = 1e5 as usize;
    let mut x = F::from_u32(10).unwrap();
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

