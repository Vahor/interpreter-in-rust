use std::fmt::{Display};
use ast::expression::Expression;
use ast::statement::{BlockStatement};
use crate::environment::Environment;

pub trait Object {
    fn inspect(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    Null,

    Integer(i64),
    Boolean(bool),
    Return(Box<ObjectType>),

    Function {
        parameters: Vec<Expression>,
        body: BlockStatement,
        environment: Environment,
    },
}


impl Object for ObjectType {
    fn inspect(&self) -> String {
        match self {
            ObjectType::Null => "null".to_string(),
            ObjectType::Integer(i) => format!("{}", i),
            ObjectType::Boolean(b) => format!("{}", b),
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
        }
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}