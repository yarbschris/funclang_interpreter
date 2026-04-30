use crate::ast::{BinaryOpcode, Expr, UnaryOpcode};
use crate::value::{Env, EvalError, Value};
use std::rc::Rc;

pub fn eval(expr: &Expr, env: Rc<Env>) -> Result<Value, EvalError> {
    match expr {
        Expr::Num(n) => Ok(Value::Int(*n)),
        Expr::Bool(b) => todo!(),
        Expr::Var(name) => todo!(),
        Expr::UnaryOp(op, e) => todo!(),
        Expr::BinaryOp(l, op, r) => todo!(),
        Expr::If(cond, thenBranch, elseBranch) => todo!(),
        Expr::Lambda { param, body } => todo!(),
        Expr::RecLambda { name, param, body } => todo!(),
        Expr::LetIn { name, value, body } => todo!(),
        Expr::App(func, arg) => todo!(),
    }
}

pub fn apply_unop(op: &UnaryOpcode, v: Value) -> Result<Value, EvalError> {
    todo!()
}

pub fn apply_binop(op: &BinaryOpcode, l: Value, r: Value) {
    todo!()
}
