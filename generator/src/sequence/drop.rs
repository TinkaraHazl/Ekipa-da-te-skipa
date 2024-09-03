use crate::Range;
use super::models::Sequence;

pub struct Drop<S: Sequence<f64>> {
    seq: S,
    d : usize
}

impl<S: Sequence<f64>> Drop<S> {
    pub fn new(seq: S, d: usize) -> Box<Drop<S>> {
        Box::new(Drop{ seq, d })
    }
    

    
    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from + self.count as u64;
        while k <= range.to + self.count as u64{
            result.push(self.sequence.k_th(k as usize));
            k += range.step;
        }
        result
    }
}

impl<S: Sequence<f64>> Sequence<f64> for Drop<S> {
    fn k_th(&self, k: usize) -> f64 {
        self.seq.k_th(k + self.d)
    }
}