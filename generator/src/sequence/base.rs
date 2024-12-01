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

fn b_to_ten(t: f64, b: usize) -> f64 {
    if t == 0.0 {
        return 0.0;
    }
    
    let mut result: f64 = 0.0;
    let mut number = t.abs();
    
    let mut digits = Vec::new();
    while number >= 1.0 {
        digits.push((number % 10.0).floor());
        number = (number / 10.0).floor();
    }
    
    for (i, &digit) in digits.iter().enumerate() {
        result += digit * (b as f64).powi(i as i32);
    }
    
    if t < 0.0 {
        result = -result;
    }
    
    result
}

fn ten_to_b(t: f64, n: usize) -> f64 {
    if t == 0.0 {
        return 0.0;
    }
    
    let mut number = t.abs().floor() as i64;
    let mut digits = Vec::new();
    
    while number > 0 {
        digits.push(number % n as i64);
        number /= n as i64;
    }
    
    let mut result = 0.0;
    for (i, &digit) in digits.iter().enumerate() {
        result += (digit as f64) * 10.0_f64.powi(i as i32);
    }
    
    if t < 0.0 {
        result = -result;
    }
    
    result
}

//#[cfg(test)]
//mod tests {
//    #[test]
//    fn ten_to_b() {
//        let result = ten_to_b(3.0, 2);
//        assert_eq!(result, 11.0)
//    }
//}