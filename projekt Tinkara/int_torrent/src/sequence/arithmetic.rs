use super::models::Sequence;
// Implementirajte artimetično zaporedje
pub struct Arithmetic<T> {
    zacetna : T,
    razlika : T
}

impl<i64> Arithmetic<i64> {
    pub fn new(zacetna: i64, razlika: i64) -> Arithmetic<i64> {
        Self {
        zacetna : zacetna,
        razlika : razlika,
        }
    }
}

impl Sequence<i64> for Arithmetic<i64> {
    fn name(&self) -> String {
        let ime = format!("{} + n * {}", self.zacetna, self.razlika);
        return ime
    }

    fn start(&self) -> i64 {
        return self.zacetna
    }

    fn k_th(&self, k: usize) -> Option<i64> {
        return Some((self.zacetna + (k as i64) * self.razlika).try_into().unwrap())
    }

    fn contains(&self, item: i64) -> bool {
        return (item - self.zacetna) % self.razlika == 0
    }
}