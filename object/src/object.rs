use std::fmt::{Display};

pub trait Object {
    fn inspect(&self) -> String;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObjectType {
    Null,

    Integer(i64),
    Boolean(bool),
}


impl Object for ObjectType {
    fn inspect(&self) -> String {
        match self {
            ObjectType::Null => "null".to_string(),
            ObjectType::Integer(i) => format!("{}", i),
            ObjectType::Boolean(b) => format!("{}", b),
        }
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}