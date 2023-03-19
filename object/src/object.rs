
pub trait Object {
    fn inspect(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
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