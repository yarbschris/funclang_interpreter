use crate::ast::Expr;
use std::rc::Rc;

// Closures in Value store a pointer to the chain of Envs that
// was visible at creation (all parents of the Env). This allows us
// to cleanly keep track of scope
#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Closure {
        param: String,
        body: Box<Expr>,
        env: Rc<Env>, // Scope Capture
    },
    RecClosure {
        name: String, // Recursive Closure stores its own name
        param: String,
        body: Box<Expr>,
        env: Rc<Env>, // Scope Capture
    },
}

// Env: A recursive data structure that allows us to keep track
// of variables
// Can think of as a backwards linked list of frames, every frame is one binding,
#[derive(Debug, Clone)]
pub enum Env {
    Empty,
    Frame {
        name: String,
        value: Value,
        parent: Rc<Env>,
    },
}

enum EvalError {
    UnboundVar(String),
}
