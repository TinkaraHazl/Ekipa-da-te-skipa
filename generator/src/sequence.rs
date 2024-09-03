//enum Sequence {
//    Geometric,
//    Constant,
//    Arithmetic,
//    Product,
//    Sum,
//    Product,
//    Drop,
//}

pub mod geometric;
pub mod constant;
pub mod arithmetic;
pub mod product;
pub mod sum;
pub mod base;
pub mod drop;
pub mod aliquot;

pub trait Sequence<T> {
    //fn name(&self) -> String;
    //fn start(&self) -> T;

    // To pustimo do naslednjič, ko se bom natančneje poučili o Rustovih traitih in izposojanju
    // fn current_index(&self) -> usize;
    // fn current(&self) -> Option<T>;

    // fn next(&mut self) -> Option<T>;
    // fn k_next(&mut self, k: usize) -> Option<T>;

    fn k_th(&self, k: usize) -> f64;

    //fn contains(&self, item: T) -> bool;
}
pub mod models;
pub mod drop;
pub mod tribonacci;
pub mod catalan;
pub mod lah;
pub mod aux_fun;
pub mod mix;