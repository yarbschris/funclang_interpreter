use crate::ast::{BinaryOpcode, Expr, UnaryOpcode};
use crate::value::{Env, EvalError, Value, ValueType};
use std::rc::Rc;

pub fn eval(expr: &Expr, env: Rc<Env>) -> Result<Value, EvalError> {
    match expr {
        Expr::Num(n) => Ok(Value::Int(*n)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Var(name) => match env.lookup(&name) {
            Some(val) => Ok(val),
            None => Err(EvalError::UnboundVar(name.clone())),
        },
        Expr::UnaryOp(op, e) => match eval(&e, env) {
            Ok(v) => apply_unop(op, v),
            Err(e) => Err(e),
        },
        Expr::BinaryOp(l, op, r) => match (eval(&l, env.clone()), eval(&r, env)) {
            (Ok(lval), Ok(rval)) => apply_binop(op, lval, rval),
            (Err(e), _) => Err(e),
            (Ok(_), Err(e)) => Err(e),
        },
        Expr::If(cond, then_branch, else_branch) => match eval(&cond, env.clone()) {
            Ok(b) => match b {
                Value::Bool(bval) => match bval {
                    true => eval(&then_branch, env),
                    false => eval(&else_branch, env),
                },
                other => Err(EvalError::MismatchedType {
                    expected: ValueType::Bool,
                    got: other.type_of(),
                }),
            },
            Err(e) => Err(e),
        },
        Expr::Lambda { param, body } => Ok(Value::Closure {
            param: param.clone(),
            body: body.clone(),
            env,
        }),
        Expr::RecLambda { name, param, body } => Ok(Value::RecClosure {
            name: name.clone(),
            param: param.clone(),
            body: body.clone(),
            env,
        }),
        Expr::LetIn { name, value, body } => {
            let value = eval(value, env.clone())?;
            let new_env = env.extend(name.clone(), value);
            eval(body, new_env)
        }
        Expr::App(func, arg) => {
            let f = eval(&func, env.clone())?;
            let a = eval(&arg, env)?;

            match (f, a) {
                (Value::Closure { param, body, env }, argument) => {
                    let new_env = env.extend(param, argument);
                    eval(&body, new_env)
                }
                (
                    Value::RecClosure {
                        name,
                        param,
                        body,
                        env,
                    },
                    argument,
                ) => {
                    let self_value = Value::RecClosure {
                        name: name.clone(),
                        param: param.clone(),
                        body: body.clone(),
                        env: Rc::clone(&env),
                    };
                    let extended_env = env.extend(name, self_value);
                    let extended_env = extended_env.extend(param, argument);
                    eval(&body, extended_env)
                }
                (other, _) => Err(EvalError::MismatchedType {
                    expected: ValueType::Function,
                    got: other.type_of(),
                }),
            }
        }
    }
}

pub fn apply_unop(op: &UnaryOpcode, v: Value) -> Result<Value, EvalError> {
    match (op, v) {
        (UnaryOpcode::Neg, Value::Int(n)) => Ok(Value::Int(-n)),
        (UnaryOpcode::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (UnaryOpcode::Neg, other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (UnaryOpcode::Not, other) => Err(EvalError::MismatchedType {
            expected: ValueType::Bool,
            got: other.type_of(),
        }),
    }
}

pub fn apply_binop(op: &BinaryOpcode, l: Value, r: Value) -> Result<Value, EvalError> {
    match (op, l, r) {
        // ADDITION
        (BinaryOpcode::Add, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
        (BinaryOpcode::Add, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::Add, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // SUBTRACTION
        (BinaryOpcode::Sub, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
        (BinaryOpcode::Sub, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::Sub, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // MULTIPLICATION
        (BinaryOpcode::Mul, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
        (BinaryOpcode::Mul, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::Mul, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // DIVISION
        (BinaryOpcode::Div, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l / r)),
        (BinaryOpcode::Div, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::Div, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // MOD
        (BinaryOpcode::Mod, Value::Int(l), Value::Int(r)) => Ok(Value::Int(l % r)),
        (BinaryOpcode::Mod, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::Mod, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // GREATER THAN
        (BinaryOpcode::GT, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l > r)),
        (BinaryOpcode::GT, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::GT, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // GREATER THAN OR EQUAL
        (BinaryOpcode::GTE, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l >= r)),
        (BinaryOpcode::GTE, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::GTE, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // LESS THAN
        (BinaryOpcode::LT, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l < r)),
        (BinaryOpcode::LT, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::LT, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // LESS THAN OR EQUAL
        (BinaryOpcode::LTE, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l <= r)),
        (BinaryOpcode::LTE, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::LTE, other, _) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // EQUAL
        (BinaryOpcode::EE, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l == r)),
        (BinaryOpcode::EE, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::EE, Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
        (BinaryOpcode::EE, Value::Bool(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Bool,
            got: other.type_of(),
        }),
        (BinaryOpcode::EE, other, _) => Err(EvalError::MismatchedType {
            // TODO: Could be int or bool
            expected: ValueType::Int,
            got: other.type_of(),
        }),

        // NOT EQUAL
        (BinaryOpcode::NE, Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l != r)),
        (BinaryOpcode::NE, Value::Int(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Int,
            got: other.type_of(),
        }),
        (BinaryOpcode::NE, Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l != r)),
        (BinaryOpcode::NE, Value::Bool(_), other) => Err(EvalError::MismatchedType {
            expected: ValueType::Bool,
            got: other.type_of(),
        }),
        (BinaryOpcode::NE, other, _) => Err(EvalError::MismatchedType {
            // TODO: Could be int or bool
            expected: ValueType::Int,
            got: other.type_of(),
        }),
    }
}
