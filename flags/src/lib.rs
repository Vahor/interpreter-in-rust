use std::sync::atomic::AtomicBool;

pub static STOP_AT_FIRST_ERROR: AtomicBool = AtomicBool::new(false);
pub static PRINT_EVALUATED_RESULT: AtomicBool = AtomicBool::new(false);