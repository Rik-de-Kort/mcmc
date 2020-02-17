use std::cmp::PartialOrd;
pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y { x } else { y }
}

use num::Float;
pub fn exp<T: Float>(x: T) -> T { x.exp() }

pub fn mean(v: &[f64]) -> f64 {
    let sum = v.iter().sum::<f64>();
    let n = v.len() as f64;
    sum/n
}

pub fn std(v: &[f64]) -> f64{
    let mean = mean(&v);
    let diffs = v.iter().map(|x| (x - mean).powi(2));
    let sum = diffs.into_iter().sum::<f64>();
    sum / v.len() as f64
}


