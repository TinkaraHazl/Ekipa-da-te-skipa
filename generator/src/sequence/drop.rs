use crate::sequence::Sequence;

pub struct Drop<S: Sequence<f64>> {
    seq: S,
    d : usize
}

impl<S: Sequence<f64>> Drop<S> {
    pub fn new(seq: S, d: usize) -> Box<Drop<S>> {
        Box::new(Drop{ seq, d })
    }
}

impl<S: Sequence<f64>> Sequence<f64> for Drop<S> {
    fn k_th(&self, k: usize) -> f64 {
        self.seq.k_th(k + self.d)
    }
}