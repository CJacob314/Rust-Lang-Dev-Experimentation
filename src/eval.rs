use crate::{Expr, Num};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("unexpected token `{0}`")]
    UnexpectedToken(String)
}

// BEAUTIFUL RECURSION!
pub fn eval(expr: &Expr) -> Result<Num, EvalError> {
    match expr {
        Expr::Number(x) => Ok(x.clone()),
        Expr::Neg(a) => Ok(-eval(a)?),
        Expr::Add(a, b) => Ok(eval(a)? + eval(b)?),
        Expr::Sub(a, b) => Ok(eval(a)? - eval(b)?),
        Expr::Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Expr::Div(a, b) => Ok(eval(a)? / eval(b)?),
        _ => todo!(), // I'll add handling for the other (more complicated) stuff later
    }
}
