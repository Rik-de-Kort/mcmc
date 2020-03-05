use ndarray_rand::rand::{thread_rng, Rng};

pub struct MetroChain<D> {
    pub x: Vec<f64>,
    pub pd: D,
}

pub trait MetroProposal {
    // Sample conditional on x
    fn sample<R: Rng>(&self, x: &[f64], rng: &mut R) -> Vec<f64>;
    // Conditional density function, p(x | y)
    fn pdf(&self, x: &[f64], y: &[f64]) -> f64;
    // Proportionality for final distribution
    fn pi(&self, x: &[f64]) -> f64;
}

impl<D: MetroProposal> Iterator for MetroChain<D> {
    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rng = thread_rng();

        let c = self.pd.sample(&self.x, &mut rng);

        let alpha = self.pd.pi(&c) / self.pd.pi(&self.x) * self.pd.pdf(&self.x, &c)
            / self.pd.pdf(&c, &self.x);
        let u: f64 = rng.gen();

        if u <= alpha {
            self.x = c
        }
        Some(self.x.clone())
    }
}
