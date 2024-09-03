use crate::Range;
use crate::sequence::Sequence;

struct Aliquot {
    a : usize
}

impl Aliquot {
    pub fn new(a: usize) -> Box<Aliquot> {
        Box::new(Aliquot {a})
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

impl Sequence<f64> for Aliquot {
    fn k_th(&self, k: usize) -> f64 {
        let mut a = self.a;
        for n in 0..k {
            a = aliquot_sum(a)
        }
        a as f64
    }
}

fn aliquot_sum(a: usize) -> usize {
    (1..(a / 2 + 1)).filter(|i| a % i == 0).sum()
}

