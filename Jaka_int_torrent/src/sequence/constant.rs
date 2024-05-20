use crate::sequence::models::Sequence;

// Implementirajte konstantno zaporedje
pub struct Constant {
    name: String,
    start: i64
}

impl Constant {
    pub fn new(name: String, start: i64) -> Self {
        Self {
            name: name,
            start: start
        }
    }
}

impl Sequence<i64> for Constant {
    fn contains(&self, item: i64) -> bool {
        item == self.start
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn k_th(&self, k: usize) -> Option<i64>{
        Some(self.start)
    }

    fn start(&self) -> Option<i64> {
        Some(self.start)
    }
}
