use std::fmt::Display;

use crate::statement::{BlockStatement};

#[derive(Debug)]
pub struct Program {
    pub statements: BlockStatement,
}

impl Default for Program {
    fn default() -> Self {
        return Self { statements: vec![] };
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for stmt in &self.statements {
            output.push_str(&format!("{}\n", stmt));
        }
        return write!(f, "{}", output);
    }
}


#[cfg(test)]
mod tests {
    use crate::statement::{Statement};
    use crate::expression::Expression;

    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![
                Statement::LetStatement{
                    identifier: "myVar".to_string(),
                    value: Expression::IntegerLiteral(5),
                },
                Statement::ReturnStatement {
                    value: Expression::IntegerLiteral(10),
                },
                Statement::ExpressionStatement(Expression::IntegerLiteral(5)),
            ],
        };

        assert_eq!(
            program.to_string(),
            "let myVar = 5;
return 10;
5;
"
        );
    }
}

