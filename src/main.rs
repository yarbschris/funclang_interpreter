pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub funclang);

fn main() {
    let inputs = [
        "let x = 1 in x + 2",
        "let x = if x == 0 then 1 else 2 in x",
        "let x = let y = 1 in y in x + 1",
        "1 + (let x = 5 in x)",
        "1 - 2 - 3",
        "let x = in 5",
        "fun x -> x + 1",
        "fun x -> fun y -> x + y",
        "fun x y -> x + y",
        "let inc = fun x -> x + 1 in inc",
        "1 + (fun x -> x)",
        "f x",
        "f x y",
        "f x + g y",
        "f (fun x ->x + 1)",
        "(fun x -> x + 1) 5",
        "let f x y = e1 in e2",
        "let f = fun x y -> e1 in e2",
        "let x = 1 in let f = fun x y -> x + y in f x 2",
    ];

    let parser = funclang::ExprParser::new();
    for src in inputs {
        match parser.parse(src) {
            Ok(ast) => println!("{src}\n=> {ast:#?}\n"),
            Err(e) => println!("{src}\n=> parse error: {e}\n"),
        }
    }
}
