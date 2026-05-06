use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Lambda {
        param: String,
        body: Box<Expr>,
    },
    RecLambda {
        name: String,
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
    Nil,
    Cons(Box<Expr>, Box<Expr>),
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<(Pattern, Box<Expr>)>,
    },
}

#[derive(Debug, Clone)]
pub enum Pattern {
    PVar(String),
    PWildcard,
    PNum(i32),
    PBool(bool),
    PNil,
    PCons(Box<Pattern>, Box<Pattern>),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOpcode {
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

        Expr::RecLambda { name, param, body } => {
            writeln!(f, "{prefix}{connector}RecLambda({name}, {param})")?;
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

        Expr::Nil => writeln!(f, "{prefix}{connector}Nil"),

        Expr::Cons(l, r) => {
            writeln!(f, "{prefix}{connector}Cons")?;
            print_tree(l, f, &child_prefix, false)?;
            print_tree(r, f, &child_prefix, true)
        }
        Expr::Match { scrutinee, arms } => {
            writeln!(f, "{prefix}{connector}Match")?;
            let scrutinee_last = arms.is_empty();
            print_tree(scrutinee, f, &child_prefix, scrutinee_last)?;
            if let Some((last_arm, rest)) = arms.split_last() {
                for (pat, body) in rest {
                    print_arm(pat, body, f, &child_prefix, false)?;
                }
                let (pat, body) = last_arm;
                print_arm(pat, body, f, &child_prefix, true)?;
            }
            Ok(())
        }
    }
}

fn print_arm(
    pat: &Pattern,
    body: &Expr,
    f: &mut fmt::Formatter<'_>,
    prefix: &str,
    last: bool,
) -> fmt::Result {
    let connector = if last { "└── " } else { "├── " };
    let child_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });
    writeln!(f, "{prefix}{connector}Arm")?;
    print_pattern(pat, f, &child_prefix, false)?;
    print_tree(body, f, &child_prefix, true)
}

fn print_pattern(
    pat: &Pattern,
    f: &mut fmt::Formatter<'_>,
    prefix: &str,
    last: bool,
) -> fmt::Result {
    let connector = if last { "└── " } else { "├── " };
    let child_prefix = format!("{}{}", prefix, if last { "    " } else { "│   " });
    match pat {
        Pattern::PVar(name) => writeln!(f, "{prefix}{connector}PVar({name})"),
        Pattern::PWildcard => writeln!(f, "{prefix}{connector}PWildcard"),
        Pattern::PNum(n) => writeln!(f, "{prefix}{connector}PNum({n})"),
        Pattern::PBool(b) => writeln!(f, "{prefix}{connector}PBool({b})"),
        Pattern::PNil => writeln!(f, "{prefix}{connector}PNil"),
        Pattern::PCons(l, r) => {
            writeln!(f, "{prefix}{connector}PCons")?;
            print_pattern(l, f, &child_prefix, false)?;
            print_pattern(r, f, &child_prefix, true)
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Expr")?;
        print_tree(self, f, "", true)
    }
}
