use crate::parser::ast::{Expr, BinOp, Literal};
use crate::runtime::environment::Environment;

pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new(environment: Environment) -> Self {
        Evaluator { environment }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Literal, String> {
        match expr {
            Expr::Literal(lit) => Ok(lit.clone()),
            Expr::Variable(name) => {
                if let Some(value) = self.environment.get(name) {
                    Ok(Literal::Number(value))
                } else {
                    Err(format!("Undefined variable '{}'", name))
                }
            }
            Expr::Binary(left, op, right) => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match (left_val, right_val) {
                    (Literal::Number(left_num), Literal::Number(right_num)) => {
                        match op {
                            BinOp::Add => Ok(Literal::Number(left_num + right_num)),
                            BinOp::Sub => Ok(Literal::Number(left_num - right_num)),
                            BinOp::Mul => Ok(Literal::Number(left_num * right_num)),
                            BinOp::Div => Ok(Literal::Number(left_num / right_num)),
                        }
                    }
                    _ => Err("Unsupported operation".to_string()),
                }
            }
        }
    }
}
