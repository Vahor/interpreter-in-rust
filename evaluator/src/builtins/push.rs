use environment::object::ObjectType;
use error::EvaluatorError;

pub fn push(args: &Vec<&mut ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if args.len() == 1 {
        return Err(EvaluatorError::missing_argument(2));
    }
    let first = &mut args[0].to_owned();
    match first {
        ObjectType::Array(arr) => push_array(arr, args[1].clone()),
        _ => Err(EvaluatorError::argument_type_not_supported("push", first.to_string().as_str())),
    }
}

fn push_array(arr: &mut Vec<ObjectType>, obj: ObjectType) -> Result<ObjectType, EvaluatorError> {
    arr.push(obj);
    Ok(ObjectType::Array(arr.to_vec()))
}

