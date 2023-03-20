use environment::object::ObjectType;
use error::EvaluatorError;

pub fn first(args: Vec<ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if args.len() != 1 {
        return Err(EvaluatorError::wrong_number_of_arguments(1, args.len()));
    }
    let first = &args[0];
    match first {
        ObjectType::Array(arr) => {
            if arr.len() > 0 {
                Ok(arr[0].clone())
            } else {
                Ok(ObjectType::Null)
            }
        }
        _ => Err(EvaluatorError::argument_type_not_supported("first", first.to_string().as_str())),
    }
}

