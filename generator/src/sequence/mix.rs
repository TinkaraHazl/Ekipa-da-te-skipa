use crate::sequence::Sequence;

pub struct Mix {
    seq1: Box<dyn Sequence>,  
    seq2: Box<dyn Sequence>, 
    step: f64
}



impl Mix {
    pub fn new(seq1: Box<dyn Sequence>, seq2: Box<dyn Sequence>, step: f64) -> Box<Mix> {
        if (step as usize) < 1 {
            panic!("Step must be greater or equal to 1.")
        }
        Box::new(Mix { seq1, seq2, step })
    }
}

impl Sequence for Mix {
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