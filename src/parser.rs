use chumsky::prelude::*;
use crate::{Expr, Num};

pub(crate) fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let num = num_parser().padded();

        let atom = num.or(expr.delimited_by(just('('), just(')'))).padded();
        
        let op = |c| just(c).padded();

        let neg = op('-')
            .repeated()
            .then(atom)
            .foldr(|_, rhs| Expr::Neg(Box::new(rhs)));

        let product = neg.clone()
            .then(op('*').to(Expr::Mul as fn(_, _) -> _)
                .or(op('/').to(Expr::Div as fn(_, _) -> _))
                .then(neg)
                .repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        product.clone()
            .then(op('+').to(Expr::Add as fn(_, _) -> _)
                .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                .then(product)
                .repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
    }).then_ignore(end())
}

pub(crate) fn num_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    let integer = text::int(10);
    let fractional = just('.').ignored().then(integer).or_not();

    integer.then(fractional).map(|(integral, fractional_op)| {
        Expr::Number(if let Some((_, fractional)) = fractional_op {
            let mut s = integral;
            s += ".";
            s += &fractional;
            Num::Float(s.parse().unwrap())
        } else {
            Num::Int(integral.parse().unwrap())
        })
    })
}
