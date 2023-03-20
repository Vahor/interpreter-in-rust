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

    #[error("Wrong number of arguments for `{function}`: expected {expected}, got {actual}")]
    WrongNumberOfArguments {
        function: String,
        expected: usize,
        actual: usize,
    },

    #[error("Required argument {index} is missing")]
    MissingArgument {
        index: usize,
    },

    #[error("Argument to `{function}` not supported, got {actual}")]
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

    #[error("{index} is out of bounds for array of size {size}")]
    IndexOutOfBounds {
        index: i64,
        size: usize,
    },

    #[error("No such key {key} in hash")]
    NoSuchKey {
        key: String,
    },

    #[error("Key not supported: {actual}")]
    KeyNotSupported {
        actual: String,
    },

    #[error("Cannot convert object `{actual}` to expression")]
    CannotConvertObjectToExpression {
        actual: String,
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

    pub fn wrong_number_of_arguments2(function: &str, expected: usize, actual: usize) -> EvaluatorError {
        EvaluatorError::WrongNumberOfArguments {
            function: function.to_string(),
            expected,
            actual,
        }

    }
    pub fn wrong_number_of_arguments(expected: usize, actual: usize) -> EvaluatorError {
        EvaluatorError::WrongNumberOfArguments {
            function: "unknown".to_string(),
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

    pub fn index_out_of_bounds(index: i64, size: usize) -> EvaluatorError {
        EvaluatorError::IndexOutOfBounds {
            index,
            size,
        }
    }

    pub fn missing_argument(index: usize) -> EvaluatorError {
        EvaluatorError::MissingArgument {
            index,
        }
    }

    pub fn no_such_key(key: String) -> EvaluatorError {
        EvaluatorError::NoSuchKey {
            key,
        }
    }

    pub fn key_not_supported(actual: String) -> EvaluatorError {
        EvaluatorError::KeyNotSupported {
            actual,
        }
    }

    pub fn conversion_error(actual: String) -> EvaluatorError {
        EvaluatorError::CannotConvertObjectToExpression {
            actual,
        }
    }

    pub fn unknown_error() -> EvaluatorError {
        EvaluatorError::UnknownError
    }
}