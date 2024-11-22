use crate::sequence::Sequence;

pub struct Base {
    seq : Box<dyn Sequence>,
    b : usize,
    n : usize,
}

impl Base {
    pub fn new(seq: Box<dyn Sequence>, b: usize, n: usize) -> Box<Base> {
        Box::new(Base{seq, b, n})
    }
}

impl Sequence for Base {
    fn k_th(&self, k: usize) -> f64 {
        ten_to_b(b_to_ten(self.seq.k_th(k), self.b), self.n)
    }
}

fn b_to_ten (t: f64, b: usize) -> f64 {
    let mut s: f64 = 0.0;
    let mut int: f64 = t.trunc();
    let mut frac: f64 = t - t.trunc();

    let mut pi = 1.0;
    while int > 0.0 {
        let digit = int % 10.0;
        s += (digit as f64) * ((b as f64).powf(pi));
        int /= 10.0;
        pi += 1.0;
    };

    let mut pf = 1.0;
    while frac > 0.0 {
        frac *= 10.0;
        let digit  = frac.trunc();
        s += digit / ((b as f64)).powf(-pf);
        frac -= digit;
        pf += 1.0
    }

    s
}

fn ten_to_b (t: f64, n: usize) -> f64 {
    let mut s: f64 = 0.0;
    let mut int_t = t.trunc();
    let mut frac_t = t - int_t;

    while int_t > 0.0 {
        s += int_t % (n as f64);
        int_t /= n as f64
    }

    for i in 1..11 {
        s += ((frac_t * (n as f64)) / 1.0).powi(-i);
        frac_t = (frac_t * (n as f64)) % 1.0
    }
    s
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn ten_to_b() {
//        let result = ten_to_b(3.0, 2);
//        assert_eq!(result, 11.0)
//    }
//}