use crate::Range;
use crate::sequence::Sequence;


pub struct Sum<S1: Sequence, S2: Sequence> {
    seq1: S1,
    seq2: S2
}


impl<S1, S2> Sum<S1, S2>{
    pub fn new(seq1: S1, seq2: S2) -> Box<Sum<S1, S2>> {
        Box::new(Sum { seq1, seq2 })
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

impl Sequence<S1: Sequence, S2: Sequence> for Sum<S1, S2> {
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) + self.seq2.k_th(k)
    }
}