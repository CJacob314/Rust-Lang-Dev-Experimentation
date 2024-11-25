mod args;
use args::Args;
mod expressions;
pub(crate) use expressions::*;
mod parser;
use parser::*;
mod eval;
use eval::*;

use chumsky::prelude::*;
use color_eyre::eyre::Result;
use std::fs;
use std::io::{self, Read};
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Args::from_args();
    let src = match args.code_file.as_str() {
        "-" => {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s)?;
            s
        }
        _ => fs::read_to_string(args.code_file)?,
    };

    match parser().parse(src) {
        Ok(ast) => {
            dbg!(&ast);
            println!("{}", eval(&ast, &mut Vec::new())?)
        }
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }

    Ok(())
}
