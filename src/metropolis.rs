use num;
use num::Float;

use std::ops::Add;

use ndarray_rand::rand::Rng;

use crate::quality_of_life::*;


fn next<T, R>(x: T, pi: fn(&T) -> f64, proposal: &impl Fn(&mut R) -> T, rng: &mut R) -> T
where
    T: Add<Output = T> + Copy,
    R: Rng,
{
    let candidate = x + proposal(rng);

    let alpha = min(1.0, pi(&candidate) / pi(&x));
    let u: f64 = rng.gen();

    if u <= alpha {
        candidate
    } else {
        x
    }
}


pub fn metropolis<T, R>(pi: fn(&T) -> f64, proposal: &impl Fn(&mut R) -> T, rng: &mut R) -> Vec<T>
where
    T: Float,
    R: Rng,
{
    let local_next = |x: T, rng: &mut R| next(x, pi, proposal, rng);

    // Execute warmup
    let n_warmup = 1e5 as usize;
    let mut x = T::zero();  // Todo: get good initial guess
    for _ in 1..n_warmup {
        x = local_next(x, rng);
    }

    // Start running the simulation
    let n = 1e6 as usize;
    let mut result = Vec::with_capacity(n);
    result.push(x);

    for i in 1..n {
        let next_val = local_next(result[i - 1], rng);
        result.push(next_val);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand_distr::Normal;

    #[test]
    fn test_trivial() {
        assert_eq!(1.0, 1.0);
    }

    #[test]
    fn test_standard_normal() {
        // We define a standard normal distribution and check if
        // parameter estimates are accurate.
        let mut rng = rand::thread_rng();
        let proposal = Normal::new(0.0, 1.0).unwrap();
        let pi = |x: f64| -> f64 { exp(-x.powi(2)) };
        let result = metropolis(pi, &proposal, &mut rng);

        assert!(mean(&result).abs() < 0.005);
        assert!(std(&result) <= 1.1)
    }
}
