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
    let mut s: f64 = 0;
    let int: f64 = t.trunc();
    let frac: f64 = t - int;
    let mut int_str: String = int.to_string();
    let frac_str: String = frac.to_string();
    let lw: usize = int_str.chars.count();
    let lf: usize  = frac_str.chars.count();

    for i in 0..lw {
        s += int_str.pop() * b.pow(i)
    }
    for i in 3..lf {
        s += frac_str.chars().nth(i).unwrap() * b.pow(-(i - 2))
    }
}

fn ten_to_b (t: f64, n: usize) -> f64 {
    let s: f64 = 0;
    let mut int_t = t.trunc();
    let mut frac_t = t - int_t;
    while int_t > 0 {
        s += int_t % n;
        int_t /= n
    }
    for i in 1..11 {
        s += ((frac_t * n) / 1).powi(-i);
        frac_t = (frac_t * n) % 1
    }
}


//#[cfg(test)]
//mod tests {
//    #[test]
//    fn ten_to_b() {
//        assert_eq!(ten_to_b(3.0, 2.0), 11)
//    }
//}