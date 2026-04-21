pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub firstlang);

fn main() {
    assert!(
        firstlang::ExprParser::new()
            .parse("if x == 1 then 1 else 4")
            .is_ok()
    )
}
