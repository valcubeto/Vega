use std::process::exit;
// use std::sync::{ Mutex, atomic::Ordering::Relaxed };
use crate::context::Program;
use term_lab::styles::Stylize;

// pub static NOTES: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

// pub fn note(arg: &'static str) {
//   let mut notes = NOTES.lock().expect("Couldn't lock NOTES");
//   notes.push(arg);
// }

/// Quits the program with a nice error message
pub fn quit(ename: &str, msg: &str, ctx: &mut Program) -> ! {
  // let notes = NOTES.lock().expect("Couldn't lock NOTES");
  // let contents = CONTENTS.lock().expect("Couldn't lock CONTENTS");
  // let curr_file = FILE.lock().expect("Couldn't lock FILE");
  // let curr_line = LINE.load(Relaxed);
  // let curr_column = COLUMN.load(Relaxed);
  // let tok_len = TOK_LEN.load(Relaxed);

  ieprintln!(ename.error() ": " msg);
  // for note in notes.iter() {
  //   ieprintln!("Note".note() ": " note);
  // }
  // let mut lines = contents.lines();
  let line_text = lines.nth(curr_line - 1).unwrap_or_else(|| exit(1));
  if !line_text.trim().is_empty() {
    eprintln!();
    let mut padding = 0;
    for ch in line_text.chars() {
      match ch {
        '\t' => padding += 4,
        ' ' => padding += 1,
        _ => break,
      }
    }
    eprintln!("{}", &line_text[padding..]);
    eprintln!("{}{}", " ".repeat(curr_column - 1 - padding), "^".repeat(tok_len).red().bold());
  }
  eprintln!("  at {}:{}:{}", ctx.curr_file, ctx.curr_line, ctx.curr_column);
  eprintln!("  at {}:{}:{}", ctx.file, ctx.line, ctx.column);
  exit(1);
}

#[macro_export]
macro_rules! internal_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("Internal error", &format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! sys_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("System error", &format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! argument_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("Argument error", &format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! syntax_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("Syntax error", &format!($($arg),*), file!(), line!(), column!());
  }};
}
