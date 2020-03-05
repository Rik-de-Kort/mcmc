pub mod gibbs;
pub mod metropolis;
pub mod output;
pub mod quality_of_life;

pub fn point_estimate<I>(chain: I, f: impl Fn(Vec<f64>) -> f64) -> f64
where
    I: Iterator<Item = Vec<f64>>,
{
    let n = 1e7;
    chain
        .skip(1e5 as usize)
        .take(n as usize)
        .map(f)
        .sum::<f64>()
        / (n as f64)
}
