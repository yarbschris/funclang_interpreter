pub mod ast;
pub mod interpreter;
pub mod value;

use lalrpop_util::lalrpop_mod;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

use crate::interpreter::eval;
use crate::value::Env;

lalrpop_mod!(pub funclang);

fn run(parser: &funclang::ExprParser, src: &str) {
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

fn main() {
    let parser = funclang::ExprParser::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let src = fs::read_to_string(&args[1]).expect("Could not read file");
        run(&parser, &src);
        return;
    }
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
        run(&parser, &src);
    }
}
