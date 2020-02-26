use ndarray_rand::rand::Rng;

/// Proposal distribution for Gibbs sampling
/// We use the option type to indicate absence of values
pub trait ProposalDistribution<T> {
    // Sample remaining values conditional on x
    fn sample<R: Rng>(&self, x: &[Option<T>], rng: &mut R) -> Vec<T>;
    // Conditional density function, p(x | y)
    // Possible fall through if x and y are not complimentary
    // Todo: figure out some way to have an assert in here
    fn pdf(&self, x: &[T]) -> T;
}

fn vec_to_option<T: Clone>(x: &[T]) -> Vec<Option<T>> {
    x.iter().map(|item| Some(item.clone())).collect()
}

fn option_to_vec<T: Clone>(x: &[Option<T>]) -> Vec<T> {
    x.iter().map(|item| {
        match item {
            Some(u) => u.clone(),
            None => panic!("Vector contains a None value!")
        }
    }).collect()
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
fn gibbs_next<T: Clone, R: Rng>(
    x: Vec<T>,
    pd: &impl ProposalDistribution<T>,
    rng: &mut R,
) -> Vec<T>
{
    let mut result: Vec<Option<T>> = vec_to_option(&x);
    for i in 0..result.len() {
        result[i] = None;
        result = vec_to_option(&pd.sample(&result, rng));
    }
    option_to_vec(&result)
}

pub fn gibbs<T: Clone, R: Rng>(
    initial: Vec<T>,
    proposal: impl ProposalDistribution<T>,
    rng: &mut R,
) -> Vec<Vec<T>>
{
    let local_next = |x, rng: &mut R| gibbs_next(x, &proposal, rng);

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
        let next_val = local_next(result[i - 1].clone(), rng);
        result.push(next_val);
    }
    result
}

