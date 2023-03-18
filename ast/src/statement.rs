use crate::expression::Expression;

#[derive(Debug)]
pub struct LetStatementData {
    pub identifier: String,
    pub value: Expression,
}

#[derive(Debug)]
pub enum Statement {

    LetStatement(LetStatementData),

    ReturnStatement {
        value: Expression,
    },
}