use std::sync::atomic::AtomicBool;


pub static COLORING: AtomicBool = AtomicBool::new(true);
pub static DEBUG: AtomicBool = AtomicBool::new(false);
pub static QUIET: AtomicBool = AtomicBool::new(false);
