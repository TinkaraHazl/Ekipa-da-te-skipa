use super::models::Sequence;

struct Shifted<'a, T> {
    samo_da_ni_unused_variable: Box<&'a T>,
}

impl<i64> Sequence<i64> for Shifted<'_, i64> {
    fn name(&self) -> String {
        panic!("Shifted")
    }

    fn start(&self) -> T {
        panic!("Shifted")
    }

    fn k_th(&self, k: usize) -> Option<T> {
        panic!("Shifted")
    }

    fn contains(&self, item: T) -> bool {
        panic!("Shifted")
    }
}

impl<T> Shifted<'_, T> {
    fn new(shift: usize, base_sequence: &dyn Sequence<T>) -> Box<Shifted<T>> {
        panic!("Shifted")
    }
}

pub fn shifted_sequence(
    base_sequence: &dyn Sequence<i64>,
    shift: usize,
) -> Box<dyn Sequence<i64> + '_> {
    Shifted::new(shift, base_sequence)
}
