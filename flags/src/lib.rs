use std::sync::atomic::AtomicBool;

pub static STOP_AT_FIRST_ERROR: AtomicBool = AtomicBool::new(false);