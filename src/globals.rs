use std::sync::atomic::{ AtomicBool, Ordering };

pub static COLORING: AtomicBool = AtomicBool::new(true);
pub static DEBUG: AtomicBool = AtomicBool::new(false);
pub static QUIET: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub fn coloring_enabled() -> bool {
  COLORING.load(Ordering::Relaxed)
}

#[inline(always)]
pub fn debug_enabled() -> bool {
  DEBUG.load(Ordering::Relaxed)
}

#[inline(always)]
pub fn quiet_enabled() -> bool {
  QUIET.load(Ordering::Relaxed)
}
