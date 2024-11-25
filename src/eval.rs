use crate::{Expr, Num};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("cannot find variable {0} in scope")]
    VariableNotFound(String),
    #[error("last line should be something to evaluate!")]
    NothingToEval
}

// BEAUTIFUL RECURSION!
pub fn eval<'a>(expr: &'a Expr, vars: &mut Vec<(&'a String, Num)>) -> Result<Num, EvalError> {
    match expr {
        Expr::Number(x) => Ok(x.clone()),
        Expr::Neg(a) => Ok(-eval(a, vars)?),
        Expr::Add(a, b) => Ok(eval(a, vars)? + eval(b, vars)?),
        Expr::Sub(a, b) => Ok(eval(a, vars)? - eval(b, vars)?),
        Expr::Mul(a, b) => Ok(eval(a, vars)? * eval(b, vars)?),
        Expr::Div(a, b) => Ok(eval(a, vars)? / eval(b, vars)?),
        Expr::LetIdent(name) => {
            if let Some((_, val)) = vars.iter().rev().find(|(var, _)| *var == name) {
                Ok(val.clone())
            } else {
                Err(EvalError::VariableNotFound(name.to_owned()))
            }
        }
        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars)?;
            vars.push((name, rhs));
            let mut output = Err(EvalError::NothingToEval);
            if let Some(then) = then {
                output = eval(then, vars);
            }
            vars.pop();
            output
        }
        _ => todo!(),
    }
}
