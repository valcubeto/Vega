
#[macro_export]
macro_rules! expect {
  ($expr:expr => $msg:expr) => {
    match $expr {
      Some(val) => val,
      None => $msg,
    }
  }
}
