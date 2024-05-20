use super::models::Sequence;
// Implementirajte artimetično zaporedje

pub struct Arithmetic {
    name: String,
    start: i64,
    dif: i64
}

impl Arithmetic {
    pub fn new(name: String, start: i64, dif: i64) -> Self {
        Self {
            name: name,
            start: start,
            dif: dif
        }
    }
    

}

impl Sequence<i64> for Arithmetic {
    fn contains(&self, item: i64) -> bool {
        (item - self.start) % self.dif == 0
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn k_th(&self, k: usize) -> Option<i64>{
        Some(self.start + (k as i64) * self.dif)
    }

    fn start(&self) -> Option<i64> {
        Some(self.start)
    }
}