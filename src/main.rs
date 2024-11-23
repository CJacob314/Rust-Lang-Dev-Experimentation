mod args;
use args::Args;
mod expressions;
pub(crate) use expressions::*;
mod parser;
use parser::*;
mod eval;
use eval::*;

use chumsky::prelude::*;
use structopt::StructOpt;
use color_eyre::eyre::Result;
use std::io::{self, Read};
use std::fs;

fn main() -> Result<()> {
    let args = Args::from_args();
    let src = match args.code_file.as_str() {
        "-" => {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s)?;
            s
        }
        _ => {
            fs::read_to_string(args.code_file)?
        }
    };

    match parser().parse(src) {
        Ok(ast) => println!("{}", eval(&ast)?),
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }

    Ok(())
}
