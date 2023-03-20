use environment::object::ObjectType;
use error::EvaluatorError;

pub fn print(args: &Vec<&mut ObjectType>) -> Result<ObjectType, EvaluatorError> {
    args.iter().for_each(|arg| print!("{}", arg));

    Ok(ObjectType::Null)
}

pub fn println(args: &Vec<&mut ObjectType>) -> Result<ObjectType, EvaluatorError> {
    args.iter().for_each(|arg| println!("{}", arg));

    Ok(ObjectType::Null)
}

