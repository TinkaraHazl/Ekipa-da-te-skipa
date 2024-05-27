use crate::Range;

pub struct Sum {
    seq1: Sequence,
    seq2: Sequence,
}

impl Sum {
    pub fn new(seq1: Sequence, seq2: Sequence) -> Box<Sum> {
        Box::new(Sum { seq1, seq2 })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) + self.seq2.k_th(k)
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
