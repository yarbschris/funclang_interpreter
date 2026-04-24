use std::fmt;

type Program = Expr;

#[derive(Debug)]
pub enum Expr {
    Lambda {
        param: String,
        body: Box<Expr>,
    },
    LetIn {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    App(Box<Expr>, Box<Expr>),
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

// --- pretty-print helpers ---

fn print_tree(expr: &Expr, f: &mut fmt::Formatter<'_>, prefix: &str, last: bool) -> fmt::Result {
    let connector = if last { "└── " } else { "├── " };
    let child_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });

    match expr {
        Expr::Num(n) => writeln!(f, "{prefix}{connector}Num({n})"),
        Expr::Bool(b) => writeln!(f, "{prefix}{connector}Bool({b})"),
        Expr::Var(v) => writeln!(f, "{prefix}{connector}Var({v})"),

        Expr::UnaryOp(op, e) => {
            writeln!(f, "{prefix}{connector}UnaryOp({op:?})")?;
            print_tree(e, f, &child_prefix, true)
        }

        Expr::BinaryOp(l, op, r) => {
            writeln!(f, "{prefix}{connector}BinaryOp({op:?})")?;
            print_tree(l, f, &child_prefix, false)?;
            print_tree(r, f, &child_prefix, true)
        }

        Expr::If(cond, then, els) => {
            writeln!(f, "{prefix}{connector}If")?;
            print_tree(cond, f, &child_prefix, false)?;
            print_tree(then, f, &child_prefix, false)?;
            print_tree(els, f, &child_prefix, true)
        }

        Expr::Lambda { param, body } => {
            writeln!(f, "{prefix}{connector}Lambda({param})")?;
            print_tree(body, f, &child_prefix, true)
        }

        Expr::App(func, arg) => {
            writeln!(f, "{prefix}{connector}App")?;
            print_tree(func, f, &child_prefix, false)?;
            print_tree(arg, f, &child_prefix, true)
        }

        Expr::LetIn { name, value, body } => {
            writeln!(f, "{prefix}{connector}Let {name}")?;
            print_tree(value, f, &child_prefix, false)?;
            print_tree(body, f, &child_prefix, true)
        }

        Expr::Match(scrutinee, arm) => {
            writeln!(f, "{prefix}{connector}Match")?;
            print_tree(scrutinee, f, &child_prefix, false)?;
            let arm_prefix = format!("{child_prefix}    ");
            writeln!(f, "{child_prefix}└── Arm")?;
            print_tree(&arm.0, f, &arm_prefix, false)?;
            print_tree(&arm.1, f, &arm_prefix, true)
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Expr")?;
        print_tree(self, f, "", true)
    }
}
