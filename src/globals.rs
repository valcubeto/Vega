use std::sync::atomic::AtomicBool;


pub static COLORING: AtomicBool = AtomicBool::new(true);
pub static DEBUG: AtomicBool = AtomicBool::new(false);
pub static QUIET: AtomicBool = AtomicBool::new(false);

pub fn coloring_enabled() -> bool {
  COLORING.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn debug_enabled() -> bool {
  DEBUG.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn quiet_enabled() -> bool {
  QUIET.load(std::sync::atomic::Ordering::Relaxed)
}
