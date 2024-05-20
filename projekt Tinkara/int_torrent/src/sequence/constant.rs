use crate::sequence::models::Sequence;

// Implementirajte konstantno zaporedje
pub struct Constant<T> {
    vrednost : T,
}
impl<T> Constant<T> {
    pub fn new(c : T) -> Constant<T> {
        Constant {
            vrednost : c,
        }
    }
}

impl Sequence<i64> for Constant<i64> {
    fn name(&self) -> String {
        let ime = format!("{}", self.vrednost);
        return ime
    }

    fn start(&self) -> i64 {
        return self.vrednost
    }

    fn k_th(&self, k: usize) -> Option<i64> {
        return Some(self.vrednost)
    }

    fn contains(&self, item: i64) -> bool {
        return self.vrednost == item
    }
    
}