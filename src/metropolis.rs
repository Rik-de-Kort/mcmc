use crate::quality_of_life::*;
use ndarray_rand::rand::Rng;
use std::ops::Add;

pub trait ProposalDistribution<T> {
    // Sample conditional on x
    fn sample<R: Rng>(&self, x: &T, rng: &mut R) -> T;
    // Conditional density function, p(x | y)
    fn pdf(&self, x: &T, y: &T) -> f64;
}

/// Draws the next item in a Markov chain using the Metropolis-Hastings algorithm
/// In case the proposal distribution's density function satisfies p(x, c) == p(c, x),
/// this yields the Metropolis algorithm.
///
/// Arguments:
///
/// * `x`: current item in the Markov chain
/// * `pi`: non-normalized density of the distribution to approximate
/// * `proposal`: conditional sampler to draw new proposals from. We need access to the underlying
/// density to calculate the correcting ratio for MH.
/// * `rng`: random seed used for this thread.
fn metropolis_hastings_next<T, R>(
    x: T,
    pi: fn(&T) -> f64,
    pd: &impl ProposalDistribution<T>,
    rng: &mut R,
) -> T
where
    T: Copy,
    R: Rng,
{
    let c = pd.sample(&x, rng);

    let alpha = pi(&c) / pi(&x) * pd.pdf(&x, &c) / pd.pdf(&c, &x);
    let u: f64 = rng.gen();

    if u <= alpha { c } else { x }
}

pub fn metropolis<T, R>(
    initial: T,
    pi: fn(&T) -> f64,
    proposal: impl ProposalDistribution<T>,
    rng: &mut R,
) -> Vec<T>
where
    T: Add<Output = T> + Copy,
    R: Rng,
{
    let local_next = |x: T, rng: &mut R| metropolis_hastings_next(x, pi, &proposal, rng);

    // Execute warmup
    let n_warmup = 1e5 as usize;
    let mut x = initial;
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
