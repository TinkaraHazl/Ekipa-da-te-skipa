pub trait Sequence<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;

    // To pustimo do naslednjič, ko se bom natančneje poučili o Rustovih traitih in izposojanju
    // fn current_index(&self) -> usize;
    // fn current(&self) -> Option<T>;

    //fn next(&mut self) -> Option<T>;
    // fn k_next(&mut self, k: usize) -> Option<T>;

    fn k_th(&self, k: usize) -> Option<T>;

    fn contains(&self, item: T) -> bool;
}

//struct ArithSequenceIter {
//    zaporedje: ArithSequence,
//    trenutno_clen: u64,
//}
//
//impl Iterator for ArithSequenceiter {
//    type Item = i64;
//    fn next(&mut self) -> Option<Self::Item> {
//        Some(self.zaporedje.k_ti_clen(self.trenutni_clen))
//    };
//    
//}