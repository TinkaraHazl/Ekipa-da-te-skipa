use crate::sequence::Sequence;

pub struct Sum<S1, S2> {
    seq1: S1,  
    seq2: S2, }


impl<S1, S2> Sum<S1, S2> 
where
    S1: Sequence<f64>, 
    S2: Sequence<f64>,  
{
    pub fn nov(seq1: S1, seq2: S2) -> Sum<S1, S2> {
        Sum { seq1, seq2 }
    }
}

impl<S1: Sequence<f64>, S2: Sequence<f64>> Sequence<f64> for Sum<S1, S2> {
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) + self.seq2.k_th(k)
    }
}