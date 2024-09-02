use crate::Range;
use crate::sequence::aux_fun::factorial;

pub struct Catalan;



impl Catalan {
    
    pub fn k_th(&self, k: usize) -> f64 {
        factorial(2.0 * (k as f64)) / (factorial((k as f64) + 1.0) * factorial(k as f64))
    }

    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k as usize));
        }
        result
    }
}