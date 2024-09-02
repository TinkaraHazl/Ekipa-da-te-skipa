use crate::Range;
use crate::sequence::aux_fun::factorial;

pub struct Lah {
    m: f64,
}


impl Lah {
    pub fn new(m: f64) -> Box<Lah> {
        Box::new(Lah { m })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        (factorial(k as f64) / factorial(self.m as f64)).powi(2) * 
        (self.m as f64) / ((k as f64) * factorial((k as f64) - (self.m as f64)))
    }

    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }
}
