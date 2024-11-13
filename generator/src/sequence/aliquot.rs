use crate::sequence::Sequence;

pub struct Aliquot {
    a : usize
}

impl Aliquot {
    pub fn new(a: usize) -> Aliquot {
        Aliquot {a}
    }
}

impl Sequence<f64> for Aliquot {
    fn k_th(&self, k: usize) -> f64 {
        let mut a = self.a;
        for _n in 0..k {
            a = aliquot_sum(a)
        }
        a as f64
    }
}

fn aliquot_sum(a: usize) -> usize {
    let less = |x| if x < a { x } else { 0 };
    (1usize..)
        .take_while(|&x| x.checked_mul(x).map_or(false, |x2| x2 <= a))
        .filter(|x| a % x == 0)
        .map(|x| less(x) + usize::from(a / x != x) * less(a / x))
        .sum::<usize>()
}

