use crate::sequence::Sequence;

pub struct Aliquot {
    a : usize
}

impl Aliquot {
    pub fn new(a: usize) -> Aliquot {
        Aliquot {a}
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

