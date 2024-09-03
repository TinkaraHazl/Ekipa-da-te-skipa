use crate::sequence::Sequence;

pub struct Base<S: Sequence<f64>> {
    seq : S,
    b : usize,
    n : usize,
}

impl<S: Sequence<f64>> Base<S> {
    pub fn new(seq: S, b: usize, n: usize) -> Box<Base<S>> {
        Box::new(Base{seq, b, n})
    }
}

impl<S: Sequence<f64>> Sequence<f64> for Base<S> {
    fn k_th(&self, k: usize) -> f64 {
        ten_to_b(b_to_ten(self.seq.k_th(k), self.b), self.n)
    }
}

fn b_to_ten (t: f64, b: usize) -> f64 {
    let mut s: f64 = 0.0;
    let int: f64 = t.trunc();
    let frac: f64 = t - int;

    let mut int_str: String = int.to_string();
    let lw: usize = int_str.len();
    for i in 0..lw {
        s += int_str.pop().unwrap().to_digit(10).unwrap() as f64 * (b.pow(i as u32) as f64)
    }

    let frac_str: String = frac.to_string();
    let lf: usize  = frac_str.len();
    for i in 3..lf {
        s += frac_str.chars().nth(i).unwrap().to_digit(10).unwrap() as f64 / (b.pow((i - 2) as u32) as f64)
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
//        assert_eq!(ten_to_b(3.0, 2), 11.0)
//    }
//}