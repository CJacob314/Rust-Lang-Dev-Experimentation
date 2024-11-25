use crate::{Expr, Num};
use chumsky::prelude::*;

pub(crate) fn parser() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    let ident = text::ident().padded();

    let expr = recursive(|expr| {
        let num = num_parser().padded();

        // Atom is either a number, parenthesis statement, or variable/"let" identifier
        let atom = num
            .or(expr.delimited_by(just('('), just(')')))
            .or(ident.map(Expr::LetIdent))
            //.or_not()
            //.map(|opt| opt.unwrap_or(Expr::Empty))
            .padded();

        let op = |c| just(c).padded();

        // Unary negation
        let neg = op('-')
            .repeated()
            .then(atom)
            .foldr(|_, rhs| Expr::Neg(Box::new(rhs)));

        // Product
        let product = neg
            .clone()
            .then(
                op('*')
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op('/').to(Expr::Div as fn(_, _) -> _))
                    .then(neg)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        // Sum
        product
            .clone()
            .then(
                op('+')
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
    });

    let decl = recursive(|decl| {
        let let_decl = text::keyword("let")
            .ignore_then(ident)
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';'))
            .then(decl.or_not())
            .map(|((name, rhs_expr), then)| Expr::Let {
                name,
                rhs: Box::new(rhs_expr),
                then: then.map(|expr| Box::new(expr)),
            });

        let_decl.or(expr).padded()
    });

    decl.then_ignore(end())
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
