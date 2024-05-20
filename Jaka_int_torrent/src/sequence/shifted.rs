use super::models::Sequence;

pub struct Shifted<'a, T> {

    base: Box<dyn Sequence<T>>,
    samo_da_ni_unused_variable: Box<&'a T>
}

impl<T> Sequence<T> for Shifted<'_, T> {
    fn name(&self) -> String {
        self.base.name()
        //panic!("Shifted")
    }

    fn start(&self) -> T {
        self.base.start();
        //panic!("Shifted")
    }

    fn k_th(&self, k: usize) -> Option<T> {
        panic!("Shifted")
    }

    fn contains(&self, item: T) -> bool {
        //Ne še delat.
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
