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


#[cfg(test)]
mod tests {
    use super::*;
    use ::rand_distr::Normal;

    #[test]
    fn test_trivial() {
        assert_eq!(1.0, 1.0);
    }

    use std::iter::Sum;
    use num::ToPrimitive;

    fn mean<'a, T: 'a>(v: &'a [T]) -> Option<f64>
    where
        T: ToPrimitive + Sum<&'a T>
    {
        let sum = T::to_f64(&v.iter().sum::<T>())?;
        let n = f64::from_usize(v.len())?;

        Some(sum / n)
    }

    #[test]
    fn test_standard_normal() {
        // We define a standard normal distribution and check if
        // parameter estimates are accurate.
        let mut rng = rand::thread_rng();
        let proposal = Normal::new(0.0, 1.0).unwrap();
        let pi = |x: f64| -> f64 { (-x.powi(2)).exp() };
        let result = metropolis(pi, &proposal, &mut rng);
        
        assert!(mean(&result).unwrap() < 0.01);
    }
}


