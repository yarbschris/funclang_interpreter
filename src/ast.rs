type Program = Expr;

#[derive(Debug)]
pub enum Expr {
    LetIn {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    Num(i32),
    Bool(bool),
    Var(String),
    BinaryOp(Box<Expr>, BinaryOpcode, Box<Expr>),
    UnaryOp(UnaryOpcode, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Box<(Expr, Box<Expr>)>),
}

#[derive(Debug)]
pub enum BinaryOpcode {
    Mul,
    Div,
    Add,
    Sub,
    Mod,
    // Comparators
    GT,
    GTE,
    LT,
    LTE,
    EE, // ==
    NE, // !=
}

#[derive(Debug)]
pub enum UnaryOpcode {
    Inc,
    Dec,
    Neg,
    Not,
}
