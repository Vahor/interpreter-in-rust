use environment::object::ObjectType;
use error::EvaluatorError;

pub fn len(args: Vec<ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if args.len() != 1 {
        return Err(EvaluatorError::wrong_number_of_arguments(1, args.len()));
    }
    let first = &args[0];
    match first {
        ObjectType::String(s) => Ok(ObjectType::Integer(s.len() as i64)),
        _ => Err(EvaluatorError::argument_type_not_supported("len", first.to_string().as_str())),
    }
}

