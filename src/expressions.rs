use std::fmt::Display;
use std::ops;

#[allow(unused)]
#[derive(Debug, Clone)]
// The AST for my language
pub(crate) enum Expr {
    // Data types
    Number(Num),
    String(String),

    // Operators
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    // Function call
    Call(String, Vec<Expr>),

    // "let" statement
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Option<Box<Expr>>, // What comes after the `let`
    },
    LetIdent(String),

    // Function definition
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>, // What comes after the function definition
    },

    // ε
    Empty,
}

// Represents numerical types
#[derive(Debug, Clone)]
pub(crate) enum Num {
    Int(i64),
    Float(f64),
}

impl ops::Neg for Num {
    type Output = Num;

    fn neg(self) -> Self::Output {
        use Num::*;

        match self {
            Int(i) => Int(-i),
            Float(f) => Float(-f),
        }
    }
}

impl ops::Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        use Num::*;

        match (self, rhs) {
            (Int(i1), Int(i2)) => Int(i1 + i2),
            (Float(f1), Float(f2)) => Float(f1 + f2),
            (Float(f), Int(i)) => Float(f + i as f64),
            (Int(i), Float(f)) => Float(f + i as f64),
        }
    }
}

impl ops::Sub for Num {
    type Output = Num;

    fn sub(self, rhs: Self) -> Self::Output {
        use Num::*;

        match (self, rhs) {
            (Int(i1), Int(i2)) => Int(i1 - i2),
            (Float(f1), Float(f2)) => Float(f1 - f2),
            (Float(f), Int(i)) => Float(f - i as f64),
            (Int(i), Float(f)) => Float(i as f64 - f),
        }
    }
}

impl ops::Mul for Num {
    type Output = Num;

    fn mul(self, rhs: Self) -> Self::Output {
        use Num::*;

        match (self, rhs) {
            (Int(i1), Int(i2)) => Int(i1 * i2),
            (Float(f1), Float(f2)) => Float(f1 * f2),
            (Float(f), Int(i)) => Float(f * i as f64),
            (Int(i), Float(f)) => Float(f * i as f64),
        }
    }
}

impl ops::Div for Num {
    type Output = Num;

    fn div(self, rhs: Self) -> Self::Output {
        use Num::*;

        match (self, rhs) {
            (Int(i1), Int(i2)) => {
                if i2 == 0 {
                    panic!("Attempt to divide by zero");
                }
                Int(i1 / i2)
            }
            (Float(f1), Float(f2)) => {
                if f2 == 0.0 {
                    panic!("Attempt to divide by zero");
                }
                Float(f1 / f2)
            }
            (Float(f), Int(i)) => {
                if i == 0 {
                    panic!("Attempt to divide by zero");
                }
                Float(f / i as f64)
            }
            (Int(i), Float(f)) => {
                if f == 0.0 {
                    panic!("Attempt to divide by zero");
                }
                Float(i as f64 / f)
            }
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Num::*;

        match self {
            Int(val) => write!(f, "{val}"),
            Float(val) => write!(f, "{val}"),
        }
    }
}
