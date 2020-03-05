use std::cmp::PartialOrd;
pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}

use num::Float;
pub fn exp<T: Float>(x: T) -> T {
    x.exp()
}

pub fn partial_min<T: PartialOrd + Copy>(v: &[T]) -> T {
    let mut x = v[0];
    for item in v.iter().skip(1) {
        if *item < x {
            x = *item;
        }
    }
    x
}

/// "Max" function for partial orders.
pub fn partial_max<T: PartialOrd + Copy>(v: &[T]) -> T {
    let mut x = v[0];
    for item in v.iter().skip(1) {
        if *item > x {
            x = *item;
        }
    }
    x
}

pub fn mean(v: &[f64]) -> f64 {
    let sum = v.iter().sum::<f64>();
    let n = v.len() as f64;
    sum / n
}

pub fn std(v: &[f64]) -> f64 {
    let mean = mean(&v);
    let diffs = v.iter().map(|x| (x - mean).powi(2));
    let sum = diffs.sum::<f64>();
    sum / v.len() as f64
}

pub fn vec_to_option(x: &[f64]) -> Vec<Option<f64>> {
    x.iter().map(|item| Some(*item)).collect()
}

pub fn option_to_vec(x: Vec<Option<f64>>) -> Vec<f64> {
    x.iter()
        .map(|item| match item {
            Some(u) => *u,
            None => panic!("Vector contains a None value!"),
        })
        .collect()
}

use ndarray_rand::rand::Rng;
use ndarray_rand::rand_distr::Distribution;
pub fn build_sampler<T, D: Distribution<T>, R: Rng>(d: D) -> impl Fn(&mut R) -> T {
    // Force ownership of d over to the closure
    move |rng: &mut R| d.sample(rng)
}
