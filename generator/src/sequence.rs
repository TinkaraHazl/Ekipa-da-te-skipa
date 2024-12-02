pub mod geometric;
pub mod constant;
pub mod arithmetic;
pub mod product;
pub mod sum;
pub mod base;
pub mod drop;
pub mod aliquot;
pub mod tribonacci;
pub mod catalan;
pub mod lah;
pub mod aux_fun;
pub mod mix;

use crate::Range;

pub trait Sequence {
    fn k_th(&self, k: usize) -> f64;

    fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }
}
