use std::sync::atomic::{ AtomicBool, Ordering };

pub struct GlobalState {
  value: AtomicBool
}

impl GlobalState {
  #[inline]
  pub const fn new(value: bool) -> Self {
    Self {
      value: AtomicBool::new(value)
    }
  }
  #[inline]
  pub fn set(&self, value: bool) {
    self.value.store(value, Ordering::Relaxed);
  }
  #[inline]
  pub fn enabled(&self) -> bool {
    self.value.load(Ordering::Relaxed)
  }
}

pub static COLOR: GlobalState = GlobalState::new(true);
pub static DEBUG: GlobalState = GlobalState::new(false);
pub static QUIET: GlobalState = GlobalState::new(false);
