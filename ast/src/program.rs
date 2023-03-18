use crate::statement::Statement;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Default for Program {
    fn default() -> Self {
        return Self { statements: vec![] };
    }
}

