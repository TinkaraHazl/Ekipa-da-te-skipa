use crate::sequence::Sequence;

pub struct Constant {
    value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Box<Constant> {
        Box::new(Constant { value })
    }
}

impl Sequence for Constant {
    fn k_th(&self, _k: usize) -> f64 {
        self.value
    }
}