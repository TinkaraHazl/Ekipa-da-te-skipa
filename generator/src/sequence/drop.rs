use crate::sequence::Sequence;

pub struct Drop {
    seq: Box<dyn Sequence>,
    d : usize
}

impl Drop {
    pub fn new(seq: Box<dyn Sequence>, d: usize) -> Box<Drop> {
        Box::new(Drop{ seq, d })
    }
}

impl Sequence for Drop {
    fn k_th(&self, k: usize) -> f64 {
        self.seq.k_th(k + self.d)
    }
}