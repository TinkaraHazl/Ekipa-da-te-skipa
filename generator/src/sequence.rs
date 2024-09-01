//enum Sequence {
//    Geometric,
//    Constant,
//    Arithmetic,
//    Sum,
//    Product,
//    Drop,
//}

pub mod geometric;
pub mod constant;
pub mod arithmetic;
pub mod sum;
pub mod product;
pub mod drop;

pub trait Sequence{
    fn k_th(&self, k: usize) -> f64;
}