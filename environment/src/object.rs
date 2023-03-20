use std::fmt::{Debug, Display};

use ast::expression::Expression;
use ast::statement::BlockStatement;
use error::EvaluatorError;

use crate::environment::Environment;

pub trait Object {
    fn inspect(&self) -> String;
}

#[derive(Clone)]
pub enum ObjectType {
    Null,

    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Box<ObjectType>),
    Hash(Vec<(ObjectType, ObjectType)>),

    Quote(Box<Expression>),

    Function {
        parameters: Vec<Expression>,
        body: BlockStatement,
        environment: Environment,
    },

    Builtin(fn(&Vec<&mut ObjectType>) -> Result<ObjectType, EvaluatorError>),

    Array(Vec<ObjectType>),
}

impl PartialEq for ObjectType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ObjectType::Null, ObjectType::Null) => true,
            (ObjectType::Integer(i), ObjectType::Integer(j)) => i == j,
            (ObjectType::Boolean(b), ObjectType::Boolean(c)) => b == c,
            (ObjectType::String(s), ObjectType::String(t)) => s == t,
            (ObjectType::Return(obj), other) => obj.as_ref() == other,
            (ObjectType::Function { .. }, ObjectType::Function { .. }) => false,
            (ObjectType::Builtin(_), ObjectType::Builtin(_)) => false,
            (ObjectType::Array(arr), ObjectType::Array(other_arr)) => arr == other_arr,
            (ObjectType::Hash(hash), ObjectType::Hash(other_hash)) => hash == other_hash,
            (ObjectType::Quote(expr), ObjectType::Quote(other_expr)) => expr == other_expr,
            _ => false,
        }
    }

}


impl Object for ObjectType {
    fn inspect(&self) -> String {
        match self {
            ObjectType::Null => "null".to_string(),
            ObjectType::Integer(i) => format!("{}", i),
            ObjectType::Boolean(b) => format!("{}", b),
            ObjectType::String(s) => format!("{}", s),
            ObjectType::Return(obj) => obj.inspect(),
            ObjectType::Function { parameters, body, .. } => {
                let mut out = String::new();
                out.push_str("fn(");
                out.push_str(&parameters.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "));
                out.push_str(") {\n");
                for stmt in body {
                    out.push_str(&format!("{}\n\t", stmt));
                }
                out.push_str("}");
                out
            }
            ObjectType::Builtin(_) => "builtin function".to_string(),
            ObjectType::Array(arr) => {
                let mut out = String::new();
                out.push_str("[");
                out.push_str(&arr.iter().map(|o| o.inspect()).collect::<Vec<String>>().join(", "));
                out.push_str("]");
                out
            }
            ObjectType::Hash(hash) => {
                let mut out = String::new();
                out.push_str("{");
                out.push_str(&hash.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join(", "));
                out.push_str("}");
                out
            },
            ObjectType::Quote(expr) => {
                format!("quote({})", expr)
            }
        }
    }
}

impl Debug for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}