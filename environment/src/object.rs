use std::fmt::Display;

use ast::expression::Expression;
use ast::statement::BlockStatement;
use error::EvaluatorError;

use crate::environment::Environment;

pub trait Object {
    fn inspect(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    Null,

    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Box<ObjectType>),

    Function {
        parameters: Vec<Expression>,
        body: BlockStatement,
        environment: Environment,
    },

    Builtin(fn(Vec<ObjectType>) -> Result<ObjectType, EvaluatorError>),

    Array(Vec<ObjectType>),
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
        }
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}