use crate::sequence::Sequence;

pub struct Arithmetic {
    start: f64,
    step: f64,
}

impl Arithmetic {
    pub fn new(start: f64, step: f64) -> Arithmetic {
        Arithmetic { start, step }
    }
}

impl Sequence for Arithmetic {
    fn k_th(&self, k: usize) -> f64 {
        self.start + (k as f64) * self.step
    }
}