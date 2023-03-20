use environment::object::ObjectType;
use error::EvaluatorError;

pub fn push(args: Vec<ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if args.len() == 1 {
        return Err(EvaluatorError::missing_argument(2));
    }
    let first = &args[0];
    match first {
        ObjectType::Array(arr) => Ok(ObjectType::Integer(arr.len() as i64)),
        _ => Err(EvaluatorError::argument_type_not_supported("push", first.to_string().as_str())),
    }

}

