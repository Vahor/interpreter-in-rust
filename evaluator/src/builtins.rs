use environment::object::ObjectType;
use error::EvaluatorError;

fn len(args: Vec<ObjectType>) -> Result<ObjectType, EvaluatorError> {
    if args.len() != 1 {
        return Err(EvaluatorError::wrong_number_of_arguments(1, args.len()));
    }
    let first = &args[0];
    match first {
        ObjectType::String(s) => Ok(ObjectType::Integer(s.len() as i64)),
        _ => Err(EvaluatorError::argument_type_not_supported("len", first.to_string().as_str())),
    }
}

// map string to function
static BUILTINS: [(&str, ObjectType); 1] = [
    ("len", ObjectType::Builtin(len)),
];

pub fn get_builtin(name: &str) -> Option<ObjectType> {
    for (key, value) in BUILTINS.iter() {
        if key == &name {
            return Some(value.clone());
        }
    }
    None
}