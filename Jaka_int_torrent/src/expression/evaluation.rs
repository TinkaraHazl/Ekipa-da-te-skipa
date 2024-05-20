use super::models::{AExpr, BinaryOperation};
impl AExpr {
    pub fn evaluate(&self) -> i64 {
        match self {
            AExpr::Num(a) => *a,
            AExpr::Variable(_) => panic!(),
            AExpr::BinOp(a, BinOp, b) => {
                let a = a.evaluate() ;
                let b = b.evaluate();
                match BinOp {
                    Add => a + b,
                    Mul => a * b,
                    Sub => a - b,
                    Pow => panic!()
                }
            }
        };

        // Tukaj lahko predpostavite, da spremenljivke ne obstajajo
        panic!("Implement variable evaluation")
    }

    pub fn evaluate_map(
        &self,
        vars: &std::collections::HashMap<String, Option<i64>>,
    ) -> Option<i64> {
        match self {
            AExpr::Num(a) => Some(*a),
            AExpr::Variable(_) => panic!(),
            AExpr::BinOp(a, BinOp , b) => {
                let a = a.evaluate() ;
                let b = b.evaluate();
                match BinOp {
                    Add => Some(a + b),
                    Mul => Some (a * b),
                    Sub => Some(a - b),
                    Pow => panic!()
                }
            }
        }
    }
}
