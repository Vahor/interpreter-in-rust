use std::fmt::{Display};

pub trait Object {
    fn inspect(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectType {
    Null,

    Integer(i64),
    Boolean(bool),
    Return(Box<ObjectType>),
}


impl Object for ObjectType {
    fn inspect(&self) -> String {
        match self {
            ObjectType::Null => "null".to_string(),
            ObjectType::Integer(i) => format!("{}", i),
            ObjectType::Boolean(b) => format!("{}", b),
            ObjectType::Return(obj) => obj.inspect(),
        }
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inspect())
    }
}