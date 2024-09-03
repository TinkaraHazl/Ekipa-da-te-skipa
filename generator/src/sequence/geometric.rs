use crate::sequence::Sequence;

pub struct Geometric {
    start: f64,
    quot: f64,
}

impl Geometric {
    pub fn new(start: f64, quot: f64) -> Box<Geometric> {
        Box::new(Geometric { start, quot })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.start * self.quot.powi(k as i32)
    }
}

impl Sequence<f64> for Geometric {
    fn k_th(&self, k: usize) -> f64 {
        self.start * self.quot.powi(k as i32)
    }
}