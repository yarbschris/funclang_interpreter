pub mod ast;
pub mod interpreter;
pub mod value;

use lalrpop_util::lalrpop_mod;
use std::io::{self, BufRead, Write};

use crate::interpreter::eval;
use crate::value::Env;

lalrpop_mod!(pub funclang);

fn main() {
    let parser = funclang::ExprParser::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break; // EOF
        }
        let src = line.trim();
        if src.is_empty() {
            continue;
        }
        match parser.parse(src) {
            Ok(ast) => {
                print!("{ast}");
                match eval(&ast, Env::empty()) {
                    Ok(value) => println!("=> {value}"),
                    Err(e) => println!("eval error: {e:?}"),
                }
            }
            Err(e) => println!("parse error: {e}"),
        }
    }
}
