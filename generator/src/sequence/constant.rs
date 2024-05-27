use crate::Range;

pub struct Constant {
    value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Box<Constant> {
        Box::new(Constant { value })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.value
    }

    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.value);
        }
        result
    }
}