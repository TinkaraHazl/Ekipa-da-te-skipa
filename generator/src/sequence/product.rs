use crate::sequence::Sequence;

pub struct Product {
    seq1: Box<dyn Sequence>,  
    seq2: Box<dyn Sequence> }



impl Product {
    pub fn new(seq1: Box<dyn Sequence>, seq2: Box<dyn Sequence>) -> Box<Product> {
        Box::new(Product { seq1, seq2 })
    }}

impl Sequence for Product {
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) * self.seq2.k_th(k)
    }
}