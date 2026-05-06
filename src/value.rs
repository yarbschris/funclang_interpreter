use crate::ast::Expr;
use std::fmt;
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
    List(Rc<List>),
}

impl Value {
    // Pure Function
    // Get the type of a value
    pub fn type_of(&self) -> ValueType {
        match self {
            Value::Int(_) => ValueType::Int,
            Value::Bool(_) => ValueType::Bool,
            Value::Closure { .. } => ValueType::Function,
            Value::RecClosure { .. } => ValueType::Function,
            Value::List(_) => ValueType::List,
        }
    }
}

impl fmt::Display for Value {
    // Impure Function: Writes values to some f, never called directly
    // Just used for formatting output
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "<int> {n}"),
            Value::Bool(b) => write!(f, "<bool> {b}"),
            Value::Closure { .. } => write!(f, "<fun>"),
            Value::RecClosure { name, .. } => write!(f, "<rec fun> {name}"),
            Value::List(l) => write!(f, "<list> {l}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Int,
    Bool,
    Function,
    List,
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

impl Env {
    // Pure Function
    // Find the value associated with a name in the scope of the current env
    pub fn lookup(&self, name: &str) -> Option<Value> {
        match self {
            Env::Empty => None,
            Env::Frame {
                name: n,
                value: val,
                parent,
            } => {
                if name == n {
                    Some(val.clone())
                } else {
                    parent.lookup(name)
                }
            }
        }
    }

    // Pure Function
    // Construct a new env whose parent points to the current env
    pub fn extend(self: Rc<Self>, name: String, value: Value) -> Rc<Env> {
        Rc::new(Env::Frame {
            name,
            value,
            parent: self,
        })
    }

    // Impure Function
    // Helper that returns a pointer to a new empty env
    pub fn empty() -> Rc<Env> {
        Rc::new(Env::Empty)
    }
}

// Cons list structure
#[derive(Debug, Clone)]
pub enum List {
    Nil,
    Cons { head: Value, tail: Rc<List> },
}

impl fmt::Display for List {
    // Impure Function: Writes values to some f, never called directly
    // Just used for formatting output
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        fmt_cells(self, f, true)?;
        write!(f, "]")
    }
}

fn fmt_cells(list: &List, f: &mut fmt::Formatter<'_>, first: bool) -> fmt::Result {
    match list {
        List::Nil => Ok(()),
        List::Cons { head, tail } => {
            if !first {
                write!(f, "; ")?;
            }
            write!(f, "{head}")?;
            fmt_cells(tail, f, false)
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    UnboundVar(String),
    MismatchedType { expected: ValueType, got: ValueType },
    DivideByZero { numerator: i32, denominator: i32 },
    NonExhaustive,
}
