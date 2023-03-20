use environment::object::ObjectType;
use error::EvaluatorError;

pub fn rest(args: &Vec<&mut ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if args.len() != 1 {
        return Err(EvaluatorError::wrong_number_of_arguments(1, args.len()));
    }
    let first = &args[0];
    match first {
        ObjectType::Array(arr) => {
            if arr.len() > 0 {
                let mut new_arr = arr.clone();
                new_arr.remove(0);
                Ok(ObjectType::Array(new_arr))
            } else {
                Ok(ObjectType::Null)
            }
        }
        _ => Err(EvaluatorError::argument_type_not_supported("rest", first.to_string().as_str())),
    }
}


