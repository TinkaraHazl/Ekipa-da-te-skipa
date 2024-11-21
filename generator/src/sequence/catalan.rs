use crate::sequence::aux_fun::factorial;
use crate::sequence::Sequence;

pub struct Catalan;

impl Catalan {
    pub fn new() -> Box<Catalan> {
        Box::new(Catalan)
    }
}

impl Sequence for Catalan {
    fn k_th(&self, k: usize) -> f64 {
        factorial(2.0 * (k as f64)) / (factorial((k as f64) + 1.0) * factorial(k as f64))
    }
}