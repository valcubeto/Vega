use std::sync::atomic::Ordering::Relaxed;
use chrono::Local;
use term_lab::styles::Stylize;
use crate::globals::DEBUG;
use crate::context::{ Program, Position };

pub fn print_debug_msg(msg: &str, ctx: &mut Program) {
  if !DEBUG.enabled() {
    return;
  }
  let Position { line, column } = ctx.pos();
  let path = iformat!(ctx.input;? ":" line ":" column);
  let date = Local::now().format("%d-%m-%y %H:%M:%S").to_string();
  iprintln!("[" {"Debug".style().bold().cyan().build()} " at " path.bold() ", " date.bold() "]");
  iprintln!(msg);
}

#[macro_export]
macro_rules! debug_msg {
  ($($arg:expr),*) => {{
    use $crate::terminal::print_debug_msg;
    print_debug_msg(&iformat!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! debug {
  ($($arg:expr),+ $(,)?) => {{
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    #[allow(unused_mut)]
    let mut msg = vec![
      $( format!("{} = {:?}", stringify!($arg).bold(), $arg) ),+
    ];
    debug_msg!("{}", msg.join("; "));
  }};
}

#[macro_export]
macro_rules! debug_pretty {
  ($($arg:expr),+ $(,)?) => {{
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    #[allow(unused_mut)]
    let mut msg = vec![
      $( format!("{} = {:#?}", stringify!($arg).bold(), $arg) ),+
    ];
    debug_msg!("{}", msg.join("; "));
  }};
}

#[macro_export]
macro_rules! debug_display {
  ($arg:expr) => {{
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    debug_msg!("{} = {}", stringify!($arg).bold(), $arg);
  }};
}

#[macro_export]
macro_rules! debug_todo {
  ($arg:expr) => {{
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    debug_msg!("{}: {}", "TODO".warning(), $arg);
  }};
}
