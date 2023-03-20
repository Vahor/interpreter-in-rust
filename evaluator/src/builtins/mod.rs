use environment::object::ObjectType;
use crate::builtins::len::len;
use crate::builtins::first::first;
use crate::builtins::last::last;
use crate::builtins::push::push;
use crate::builtins::pop::pop;

pub mod len;
pub mod first;
pub mod last;
pub mod push;
pub mod pop;

pub fn get_builtin(name: &str) -> Option<ObjectType> {
    for (key, value) in BUILTINS.iter() {
        if key == &name {
            return Some(value.clone());
        }
    }
    None
}

// map string to function
static BUILTINS: [(&str, ObjectType); 5] = [
    ("len", ObjectType::Builtin(len)),
    ("first", ObjectType::Builtin(first)),
    ("last", ObjectType::Builtin(last)),
    ("push", ObjectType::Builtin(push)),
    ("pop", ObjectType::Builtin(pop)),
];
