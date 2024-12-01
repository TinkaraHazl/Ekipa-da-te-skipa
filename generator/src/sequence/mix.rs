use crate::sequence::Sequence;

pub struct Mix {
    seq1: Box<dyn Sequence>,  
    seq2: Box<dyn Sequence>, 
    steps1: usize,  // How many elements to take from seq1
    steps2: usize,  // How many elements to take from seq2
}

impl Mix {
    pub fn new(seq1: Box<dyn Sequence>, seq2: Box<dyn Sequence>, steps1: f64, steps2: f64) -> Box<Mix> {
        if steps1 < 1.0 || steps2 < 1.0 {
            panic!("Steps must be greater or equal to 1.")
        }
        Box::new(Mix { 
            seq1, 
            seq2, 
            steps1: steps1 as usize,
            steps2: steps2 as usize,
        })
    }
}

impl Sequence for Mix {
    fn k_th(&self, k: usize) -> f64 {
        let period = self.steps1 + self.steps2;
        let position_in_period = k % period;
        
        if position_in_period < self.steps1 {
            // We're in the first sequence
            let element_index = (k / period) * self.steps1 + position_in_period;
            self.seq1.k_th(element_index)
        } else {
            // We're in the second sequence
            let position_in_seq2 = position_in_period - self.steps1;
            let element_index = (k / period) * self.steps2 + position_in_seq2;
            self.seq2.k_th(element_index)
        }
    }
}