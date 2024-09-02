use crate::Range;

pub struct Geometric {
    start: f64,
    quot: f64,
}

impl Geometric {
    pub fn new(start: f64, quot: f64) -> Box<Geometric> {
        Box::new(Geometric { start, quot })
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.start * self.quot.powi(k as i32)
    }

    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k: f64 = range.from as f64;
        while k <= range.to as f64 {
            result.push(self.k_th(k as usize));
            k = k * self.quot;
        }
        result
    }
}
