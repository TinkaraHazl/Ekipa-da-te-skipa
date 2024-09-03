use crate::Range;
use crate::sequence::Sequence;

pub struct Sum<S1, S2> {
    zaporedje1: S1,  
    zaporedje2: S2, }


impl<S1, S2> Sum<S1, S2> 
where
    S1: Sequence<f64>, 
    S2: Sequence<f64>,  
{
    pub fn nov(zaporedje1: S1, zaporedje2: S2) -> Sum<S1, S2> {
        Sum { zaporedje1, zaporedje2 }
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.zaporedje1.k_th(k) + self.zaporedje2.k_th(k)
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



impl Sequence<S1: Sequence<f64>, S2: Sequence<f64>> for Sum<S1, S2> {
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) + self.seq2.k_th(k)
    }
}