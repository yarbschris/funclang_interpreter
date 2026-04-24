pub mod ast;

use lalrpop_util::lalrpop_mod;
use std::io::{self, BufRead, Write};

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
            Ok(ast) => print!("{ast}"),
            Err(e) => println!("parse error: {e}"),
        }
    }
}
