use crate::sequence::aux_fun::factorial;
use crate::sequence::Sequence;

pub struct Lah {
    m: f64,
}


impl Lah {
    pub fn new(m: f64) -> Box<Lah> {
        Box::new(Lah { m })
    }
}

impl Sequence<f64> for Lah {
    fn k_th(&self, k: usize) -> f64 {
    (factorial(k as f64) / factorial(self.m as f64)).powi(2) * 
    (self.m as f64) / ((k as f64) * factorial((k as f64) - (self.m as f64)))
    }
}