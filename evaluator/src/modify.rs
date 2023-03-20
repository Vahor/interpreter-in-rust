use ast::expression::Expression;
use error::EvaluatorError;

pub fn modify(expression: Option<&mut Expression>, modifier: impl Fn(&mut Expression) -> Result<(), EvaluatorError>) -> Result<(), EvaluatorError> {
    if let Some(expression) = expression {
        match expression {
            Expression::InfixExpression { left, right, .. } => {
                modifier(left)?;
                modifier(right)?;
                modifier(expression)?;
            }
            _ => modifier(expression)?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::expression::Expression;

    #[test]
    fn modify_integer() {
        let mut expression = Expression::IntegerLiteral(1);
        modify(Some(&mut expression), |expression| {
            match expression {
                Expression::IntegerLiteral(integer) => {
                    *integer = 2;
                }
                _ => {}
            }

            Ok(())
        }).unwrap();

        assert_eq!(expression, Expression::IntegerLiteral(2));
    }

    #[test]
    fn modify_infix_expression() {
        let mut expression = Expression::InfixExpression {
            left: Box::new(Expression::IntegerLiteral(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::IntegerLiteral(2)),
        };

        modify(Some(&mut expression), |expression| {
            match expression {
                Expression::InfixExpression { left, operator, right } => {
                    *left = Box::new(Expression::IntegerLiteral(69));
                    *operator = "-".to_string();
                    *right = Box::new(Expression::IntegerLiteral(420));
                }
                _ => {}
            }

            Ok(())
        }).unwrap();

        assert_eq!(expression, Expression::InfixExpression {
            left: Box::new(Expression::IntegerLiteral(69)),
            operator: "-".to_string(),
            right: Box::new(Expression::IntegerLiteral(420)),
        });
    }
}