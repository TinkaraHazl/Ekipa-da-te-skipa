use crate::sequence::Sequence;
pub struct Tribonacci {
    a0: f64,
    a1: f64,
    a2: f64,
}

impl Tribonacci {
    pub fn new(a0: f64, a1: f64, a2: f64) -> Tribonacci {
        Tribonacci { a0, a1, a2 }
    }
}

impl Sequence for Tribonacci {
    fn k_th(&self, k: usize) -> f64 {
        if k == 0 {
            self.a0
        } else if k == 1 {
            self.a1
        } else if k == 2 {
            self.a2
        } else {
            let mut prev2 = self.a0;
            let mut prev1 = self.a1;
            let mut current = self.a2;
            for _ in 3..=k {
                let next = prev2 + prev1 + current;
                prev2 = prev1;
                prev1 = current;
                current = next;
            }
            current
        }
    }
}