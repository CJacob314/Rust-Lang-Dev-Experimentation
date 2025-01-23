use crate::{Expr, Num};
use chumsky::chain::Chain;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("cannot find variable {0} in scope")]
    VariableNotFound(String),
    #[error("last line should be something to evaluate!")]
    NothingToEval,
    #[error("wrong number of arguments given for function `{0}`. Expected {1}, found {2}")]
    WrongNumArgs(String, usize, usize),
    #[error("cannot find function `{0}` in scope")]
    FunctionNotInScope(String),
}

// BEAUTIFUL RECURSION!
pub fn eval<'a>(
    expr: &'a Expr,
    vars: &mut Vec<(&'a String, Num)>,
    functions: &mut Vec<(&'a String, &'a [String], &'a Expr)>,
) -> Result<Num, EvalError> {
    match expr {
        Expr::Number(x) => Ok(x.clone()),
        Expr::Neg(a) => Ok(-eval(a, vars, functions)?),
        Expr::Add(a, b) => Ok(eval(a, vars, functions)? + eval(b, vars, functions)?),
        Expr::Sub(a, b) => Ok(eval(a, vars, functions)? - eval(b, vars, functions)?),
        Expr::Mul(a, b) => Ok(eval(a, vars, functions)? * eval(b, vars, functions)?),
        Expr::Div(a, b) => Ok(eval(a, vars, functions)? / eval(b, vars, functions)?),
        Expr::LetIdent(name) => {
            if let Some((_, val)) = vars.iter().rev().find(|(var, _)| *var == name) {
                Ok(val.clone())
            } else {
                Err(EvalError::VariableNotFound(name.to_owned()))
            }
        }
        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars, functions)?;
            vars.push((name, rhs));
            let mut output = Err(EvalError::NothingToEval);
            if let Some(then) = then {
                output = eval(then, vars, functions);
            }
            vars.pop();
            output
        }
        Expr::Call(name, args) => {
            if let Some((_, arg_names, body)) = functions
                .iter()
                .rev()
                .find(|(var, _, _)| *var == name)
                .copied()
            {
                if arg_names.len() != args.len() {
                    Err(EvalError::WrongNumArgs(
                        name.clone(),
                        arg_names.len(),
                        args.len(),
                    ))
                } else {
                    let mut args = args
                        .iter()
                        .map(|arg| eval(arg, vars, functions))
                        .zip(arg_names.iter())
                        .map(|(val, name)| Ok((name, val?)))
                        .collect::<Result<_, EvalError>>()?;
                    vars.append(&mut args);
                    let output = eval(body, vars, functions);
                    vars.truncate(vars.len() - args.len());
                    output
                }
            } else {
                Err(EvalError::FunctionNotInScope(name.clone()))
            }
        }
        Expr::Fn {
            name,
            args,
            body,
            then,
        } => {
            functions.push((name, args, body));
            let output = eval(then, vars, functions);
            functions.pop();
            output
        }
        _ => todo!(),
    }
}
