use crate::sequence::aux_fun::factorial;
use crate::sequence::Sequence;

pub struct Lah {
    m: f64,
}


impl Lah {
    pub fn new(m: f64) -> Box<Lah> {
        Box::new(Lah { m })
    }
}

impl Sequence for Lah {
    fn k_th(&self, k: usize) -> f64 {
        let m = self.m as usize;
        // Lah numbers are only defined for k >= m
        if k < m {
            return 0.0;
        }
        // L(k,m) = k! * C(k-1,m-1) / m!
        let numerator = factorial(k as f64);
        let denominator = factorial(m as f64);
        let combination = factorial((k-1) as f64) / 
                         (factorial((m-1) as f64) * factorial((k-m) as f64));
        
        (numerator * combination) / denominator
    }
}