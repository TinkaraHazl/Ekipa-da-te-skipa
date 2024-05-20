use super::models::{AExpr, BinaryOperation};
use std::collections::HashMap;

impl AExpr {
    pub fn evaluate(&self) -> i64 {{
        match self {
            AExpr::Num(k) => *k,
            AExpr::Variable(spremenljivka) => panic!("Spremenljivk ne sme biti."),
            AExpr::BinOp(levo, operacija, desno) => match operacija {
                BinaryOperation::Add => levo.evaluate() + desno.evaluate(),
                BinaryOperation::Sub => levo.evaluate() - desno.evaluate(),
                BinaryOperation::Mul => levo.evaluate() * desno.evaluate(),
                BinaryOperation::Pow => panic!("Napisat treba")
            }
        }
    };

    pub fn evaluate_map(
        &self,
        vars: &std::collections::HashMap<String, Option<i64>>,
    ) -> Option<i64> {
        match self {
            AExpr::Num(k) => Some(*k),
            AExpr::Variable(spremenljivka) => vars[spremenljivka],
            AExpr::BinOp(levo, operacija, desno) => match operacija {
                BinaryOperation::Add => Some(levo.evaluate_map().unwrap() + desno.evaluate_map().unwrap()),
                BinaryOperation::Sub => Some(levo.evaluate_map().unwrap() - desno.evaluate_map().unwrap()),
                BinaryOperation::Mul => Some(levo.evaluate_map().unwrap() * desno.evaluate_map().unwrap()),
                BinaryOperation::Pow => panic!("napisi")
        }
    }
}
}
}