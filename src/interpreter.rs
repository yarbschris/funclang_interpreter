use crate::ast::{BinaryOpcode, Expr, Pattern, UnaryOpcode};
use crate::value::{Env, EvalError, List, Value, ValueType};
use std::rc::Rc;

pub fn eval(expr: &Expr, env: Rc<Env>) -> Result<Value, EvalError> {
    match expr {
        Expr::Num(n) => Ok(Value::Int(*n)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Var(name) => match env.lookup(name) {
            Some(val) => Ok(val),
            None => Err(EvalError::UnboundVar(name.clone())),
        },
        Expr::UnaryOp(op, e) => match eval(e, env) {
            Ok(v) => apply_unop(op, v),
            Err(e) => Err(e),
        },
        Expr::BinaryOp(l, op, r) => match (eval(l, env.clone()), eval(r, env)) {
            (Ok(lval), Ok(rval)) => apply_binop(op, lval, rval),
            (Err(e), _) => Err(e),
            (Ok(_), Err(e)) => Err(e),
        },
        Expr::If(cond, then_branch, else_branch) => match eval(cond, env.clone()) {
            Ok(b) => match b {
                Value::Bool(bval) => match bval {
                    true => eval(then_branch, env),
                    false => eval(else_branch, env),
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
            let f = eval(func, env.clone())?;
            let a = eval(arg, env)?;

            match (f, a) {
                // Function Application: Extend env with parameter and argument value and evaluate
                // body
                (Value::Closure { param, body, env }, argument) => {
                    let new_env = env.extend(param, argument);
                    eval(&body, new_env)
                }
                // Recursive Function Application: In order to evaluate a body that contains the name of the
                // function that we are applying, we must extend the env with the closure itself
                // using the name of the enclosure, then we extend env with the parameter and
                // argument and evaluate the body
                (
                    Value::RecClosure {
                        name,
                        param,
                        body,
                        env,
                    },
                    argument,
                ) => {
                    // We make a complete clone of the closure because rust does not handle cycles
                    // well. This is inefficient, and I'm sure there are ways around it, but it is
                    // what it is
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
        // Return a new list value that contains a pointer to a nil list
        Expr::Nil => Ok(Value::List(Rc::new(List::Nil))),
        Expr::Cons(l, r) => {
            let l_val = eval(l, env.clone())?;
            let r_val = eval(r, env)?;
            apply_cons(l_val, r_val)
        }
        Expr::Match {
            scrutinee: s,
            arms: a,
        } => {
            let v = eval(s, Rc::clone(&env))?;
            for (pat, body) in a {
                if let Some(bindings) = try_match(&v, pat) {
                    // We use fold to extend env for every binding
                    let arm_env = bindings
                        .into_iter()
                        .fold(Rc::clone(&env), |e, (name, val)| e.extend(name, val));
                    return eval(body, arm_env);
                }
            }

            Err(EvalError::NonExhaustive)
        }
    }
}

// Pure Function: Attempt to match when pattern matching detected
pub fn try_match(value: &Value, pattern: &Pattern) -> Option<Vec<(String, Value)>> {
    match (pattern, value) {
        (Pattern::PWildcard, _) => Some(vec![]),
        (Pattern::PVar(name), v) => Some(vec![(name.clone(), v.clone())]),
        (Pattern::PNum(n), Value::Int(m)) => {
            if n == m {
                Some(vec![])
            } else {
                None
            }
        }
        (Pattern::PBool(b), Value::Bool(c)) => {
            if b == c {
                Some(vec![])
            } else {
                None
            }
        }
        (Pattern::PNil, Value::List(rc)) => match &**rc {
            List::Nil => Some(vec![]),
            List::Cons { .. } => None,
        },
        (Pattern::PCons(ph, pt), Value::List(rc)) => match &**rc {
            List::Nil => None,
            List::Cons { head, tail } => {
                let mut bs = try_match(head, ph)?;
                let bs_tail = try_match(&Value::List(Rc::clone(tail)), pt)?;
                bs.extend(bs_tail);
                Some(bs)
            }
        },
        _ => None,
    }
}

pub fn apply_cons(l: Value, r: Value) -> Result<Value, EvalError> {
    match (l, r) {
        (candidate, Value::List(tail_rc)) => {
            // Cons (create a new "List" whose tail points to right side)
            Ok(Value::List(Rc::new(List::Cons {
                head: candidate,
                tail: tail_rc,
            })))
        }
        (_, other) => Err(EvalError::MismatchedType {
            expected: ValueType::List,
            got: other.type_of(),
        }),
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
        (BinaryOpcode::Div, Value::Int(l), Value::Int(r)) => match r {
            0 => Err(EvalError::DivideByZero {
                numerator: l,
                denominator: r,
            }),
            _ => Ok(Value::Int(l / r)),
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{List, Value};
    use std::rc::Rc;

    // Constructs a list from a Vector of values
    fn list_of(xs: Vec<Value>) -> Value {
        let list = xs.into_iter().rev().fold(Rc::new(List::Nil), |acc, v| {
            Rc::new(List::Cons { head: v, tail: acc })
        });
        Value::List(list)
    }

    #[test]
    fn cons_pattern_binds_head_and_tail() {
        let xs = list_of(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);

        let pat = Pattern::PCons(
            Box::new(Pattern::PVar("h".to_string())),
            Box::new(Pattern::PVar("t".to_string())),
        );

        let result = try_match(&xs, &pat);

        let bindings = result.expect("pattern should match");
        assert_eq!(bindings.len(), 2);

        // Make sure head is bound to Int(1)
        let (h_name, h_val) = &bindings[0];
        assert_eq!(h_name, "h");
        assert!(matches!(h_val, Value::Int(1)));

        // Make sure tail is bound to a list
        let (t_name, t_val) = &bindings[1];
        assert_eq!(t_name, "t");
        assert!(matches!(t_val, Value::List(_)));
    }

    #[test]
    fn nil_pattern_binds_empty_list() {
        let xs = list_of(vec![]);

        let pat = Pattern::PNil;

        let result = try_match(&xs, &pat);

        let bindings = result.expect("pattern should match");

        assert_eq!(bindings.len(), 0);
    }

    #[test]
    fn nil_pattern_no_bind_cons() {
        let xs = list_of(vec![Value::Int(1), Value::Int(2)]);
        let pat = Pattern::PNil;

        let result = try_match(&xs, &pat);

        assert!(result.is_none());
    }

    #[test]
    fn literal_match_int_no_bind() {
        let val = Value::Int(5);
        let pat = Pattern::PNum(5);

        let result = try_match(&val, &pat);

        let bindings = result.expect("pattern should match");

        assert_eq!(bindings.len(), 0);
    }

    #[test]
    fn literal_match_bool_no_bind() {
        let val = Value::Bool(true);
        let pat = Pattern::PBool(true);

        let result = try_match(&val, &pat);

        let bindings = result.expect("pattern should match");

        assert_eq!(bindings.len(), 0);
    }

    #[test]
    fn literal_mismatch_no_bind_int() {
        let val = Value::Int(4);
        let pat = Pattern::PNum(5);

        let result = try_match(&val, &pat);

        assert!(result.is_none());
    }

    #[test]
    fn literal_mismatch_no_bind_bool() {
        let val = Value::Bool(true);
        let pat = Pattern::PBool(false);

        let result = try_match(&val, &pat);

        assert!(result.is_none());
    }
}
