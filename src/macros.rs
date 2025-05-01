
#![macro_use]

#[macro_export]
macro_rules! some {
  ($expr:expr => $msg:expr) => {
    match $expr {
      Some(val) => val,
      None => $msg,
    }
  }
}

#[macro_export]
macro_rules! ok {
  ($expr:expr => $macro:ident ! ( $($it:expr)* )) => {{
    use term_lab::styles::Stylize;
    match $expr {
      Ok(val) => val,
      Err(why) => $macro!($($it)+ "\r\n" {"Reason".error()} ": " why),
    }
  }}
}
