use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

// Todo: figure out how to retrofit this trait to distributions in rand
trait NiceDist<T> : Distribution<T>{
    fn sample_single(&self) -> T{
        let mut rng = rand::thread_rng();
        return self.sample(&mut rng)
    }
}


struct MarkovChain {
    x : f64,
}

impl Iterator for MarkovChain {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        self.x = self.x + 1.0;
        return Some(self.x);
    }
}

fn min_f64(x: f64, y: f64) -> f64 {
    if x < y{
        return x;
    } else {
        return y;
    }
}



fn main() {
    let mut rng = rand::thread_rng();

    let dist = Normal::new(0.0, 1.0).unwrap();
    let unif = Uniform::new(0.0, 1.0);

    // Does this need to be mutable because it refers to "state"?
    fn f(x: f64) -> f64 {
        return 1.0 / x.powf(2.0);
    }

    let mut next = |x: f64| -> f64 {
        let candidate = x + dist.sample(&mut rng);

        let alpha = min_f64(f(candidate) / f(x), 1.0);
        let u = unif.sample(&mut rng);
        if u <= alpha {
            return candidate;
        } else {
            return x;
        }
    };

    let mut x: f64 = 1.0;
    let n = 1000;
    for _ in 1..n{
        println!("{}", x);
        x = next(x);
    }
}
