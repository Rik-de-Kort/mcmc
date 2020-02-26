use ndarray_rand::rand::Rng;

/// Proposal distribution for Gibbs sampling
/// We use the option type to indicate absence of values
pub trait ProposalDistribution {
    // Sample remaining values conditional on x
    fn sample<R: Rng>(&self, x: &Vec<Option<f64>>, rng: &mut R) -> Vec<f64>;
    // Conditional density function, p(x | y)
    // Possible fall through if x and y are not complimentary
    // Todo: figure out some way to have an assert in here
    fn pdf(&self, x: &Vec<Option<f64>>, y: &Vec<Option<f64>>) -> f64;
}


/// Draws the next item in the chain using Gibbs sampling.
/// With Gibbs sampling, we iterate over the elements of x, sampling
/// each coordinate in turn, conditional on the previous ones.
///
/// Arguments:
///
/// * `x`: current item in the Markov chain
/// * `pi`: non-normalized density of the distribution to approximate
/// * `proposal`: conditional sampler to draw new proposals from. We need access to the underlying
/// density to calculate the correcting ratio for MH.
/// * `rng`: random seed used for this thread.
fn gibbs_next<R: Rng>(
    x: &Vec<f64>,
    pi: fn(&Vec<f64>) -> f64,
    pd: &impl ProposalDistribution,
    rng: &mut R,
) -> Vec<f64>
{
    x.clone()
}

pub fn gibbs<R: Rng>(
    initial: Vec<f64>,
    pi: fn(&Vec<f64>) -> f64,
    proposal: impl ProposalDistribution,
    rng: &mut R,
) -> Vec<Vec<f64>>
{
    let local_next = |x: &Vec<f64>, rng: &mut R| gibbs_next(x, pi, &proposal, rng);

    // Execute warmup
    let n_warmup = 1e5 as usize;
    let mut x = initial;
    for _ in 1..n_warmup {
        x = local_next(&x, rng);
    }

    // Start running the simulation
    let n = 1e6 as usize;
    let mut result = Vec::with_capacity(n);
    result.push(x);

    for i in 1..n {
        let next_val = local_next(&result[i - 1], rng);
        result.push(next_val);
    }
    return result;
}

