use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use crate::environment::Env;
use parser::ast::BlockStatement;

pub type EvalError = String;
pub type BuiltinFunc = fn(Vec<Rc<Object>>) -> Object;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Rc<Object>),
    Function(Vec<String>, BlockStatement, Env),
    Builtin(BuiltinFunc),
    Error(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(expr) => write!(f, "{}", expr),
            Object::Function(params, body,  _env) => {
                write!(f, "fn({}) {{ {} }}", params.join(", "), body)
            },
            Object::Builtin(_) => write!(f, "[builtin function]"),
            Object::Error(e) => write!(f, "{}", e),
        }
    }
}

