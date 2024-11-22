use crate::sequence::Sequence;

pub struct Sum {
    seq1: Box<dyn Sequence>,  
    seq2: Box<dyn Sequence> }


impl Sum {
    pub fn new(seq1: Box<dyn Sequence>, seq2: Box<dyn Sequence>) -> Box<Sum> {
        Box::new(Sum { seq1, seq2 })
    }
}

impl Sequence for Sum {
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) + self.seq2.k_th(k)
    }
}