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
        let period = self.step as usize;
        let sequence_index = (k / period) % 2;
        let element_index = k / 2;  // This ensures each sequence gets increasing values
        
        match sequence_index {
            0 => self.seq1.k_th(element_index),
            1 => self.seq2.k_th(element_index),
            _ => unreachable!(),
        }
    }
}