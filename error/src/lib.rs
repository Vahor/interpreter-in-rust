use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EvaluatorError {
    #[error("Operator not supported: {actual}")]
    OperatorNotSupported {
        actual: String,
    },

    #[error("Type mismatch: {expected} {operator} {actual}")]
    TypeMismatch {
        expected: String,
        operator: String,
        actual: String,
    },

    #[error("Wrong number of arguments, expected {expected}, got {actual}")]
    WrongNumberOfArguments {
        expected: usize,
        actual: usize,
    },

    #[error("Argument to {function} not supported, got {actual}")]
    ArgumentTypeNotSupported {
        function: String,
        actual: String,
    },

    #[error("{actual} is a built-in function")]
    BuiltInFunction {
        actual: String,
    },

    #[error("{actual} is a reserved keyword")]
    ReservedKeyword {
        actual: String,
    },

    #[error("Unknown identifier: {identifier}")]
    UnknownIdentifier {
        identifier: String,
    },

    #[error("Unexpected token: expected {expected:?}, got {actual:?} at line {line}, column {column}")]
    UnexpectedToken { expected: String, actual: String, line: u32, column: u32 },

    #[error("String literal not closed got {actual} at line {line}, column {column}")]
    UnfinishedString {
        actual: String,
        line: u32,
        column: u32,
    },

    #[error("Unknown error")]
    UnknownError,
}

impl EvaluatorError {
    pub fn operator_not_supported(actual: String) -> EvaluatorError {
        EvaluatorError::OperatorNotSupported {
            actual,
        }
    }

    pub fn type_missmatch(expected: &str, operator: &str, actual: &str) -> EvaluatorError {
        EvaluatorError::TypeMismatch {
            expected: expected.to_string(),
            operator: operator.to_string(),
            actual: actual.to_string(),
        }
    }

    pub fn unknown_identifier(identifier: &str) -> EvaluatorError {
        EvaluatorError::UnknownIdentifier {
            identifier: identifier.to_string(),
        }
    }

    pub fn wrong_number_of_arguments(expected: usize, actual: usize) -> EvaluatorError {
        EvaluatorError::WrongNumberOfArguments {
            expected,
            actual,
        }
    }

    pub fn argument_type_not_supported(function: &str, actual: &str) -> EvaluatorError {
        EvaluatorError::ArgumentTypeNotSupported {
            function: function.to_string(),
            actual: actual.to_string(),
        }
    }

    pub fn built_in_function(actual: &str) -> EvaluatorError {
        EvaluatorError::BuiltInFunction {
            actual: actual.to_string(),
        }
    }

    pub fn reserved_keyword(actual: &str) -> EvaluatorError {
        EvaluatorError::ReservedKeyword {
            actual: actual.to_string(),
        }
    }

    pub fn unfinished_string(actual: String, line: u32, column: u32) -> EvaluatorError {
        EvaluatorError::UnfinishedString {
            actual,
            line,
            column,
        }
    }

    pub fn expected_token(expected: &str, actual: &str, line: u32, column: u32) -> EvaluatorError {
        EvaluatorError::UnexpectedToken {
            expected: expected.to_string(),
            actual: actual.to_string(),
            line,
            column,
        }
    }

    pub fn unknown_error() -> EvaluatorError {
        EvaluatorError::UnknownError
    }
}