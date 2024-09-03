use crate::sequence::Sequence;

pub struct Mix<S1, S2> {
    seq1: S1,  
    seq2: S2, 
    step: f64,
}



impl<S1: Sequence<f64>, S2: Sequence<f64>> Mix<S1, S2> {
    fn new(seq1: S1, seq2: S2, step: f64) -> Mix<S1, S2> {
        if (step as usize) < 1 {
            panic!("Step must be greater or equal to 1.")
        }
        Mix { seq1, seq2, step }
    }
}

impl<S1: Sequence<f64>, S2: Sequence<f64>> Sequence<f64> for Mix<S1, S2> {
    fn k_th(&self, k: usize) -> f64 {
        let s = self.step as usize;
        let q = k / s;
        let n = k % s;
        if q % 2 == 0 {
            self.seq1.k_th((q / 2) * s + n - 1)
        }
        else {
            self.seq2.k_th((q / 2) * s + n - 1)
        }
    }
}