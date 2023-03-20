use environment::object::ObjectType;
use error::EvaluatorError;

pub fn pop(args: &Vec<&mut ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if args.len() != 1 {
        return Err(EvaluatorError::wrong_number_of_arguments(1, args.len()));
    }
    let first = &mut args[0].to_owned();
    match first {
        ObjectType::Array(arr) => pop_array(arr),
        _ => Err(EvaluatorError::argument_type_not_supported("pop", first.to_string().as_str())),
    }
}

fn pop_array(arr: &mut Vec<ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if arr.len() > 0 {
        Ok(arr.pop().unwrap())
    } else {
        Ok(ObjectType::Null)
    }
}

