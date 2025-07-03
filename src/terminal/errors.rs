use std::process::exit;
use crate::context::Program;
use term_lab::styles::Stylize;

/// Quits the program with a nice error message
pub fn quit(ename: &str, msg: &str, /* ctx: &mut Program, */ src_file: &str, src_line: u32, src_column: u32) -> ! {
  ieprintln!(ename.error() ": " msg);
  // for note in notes.iter() {
  //   ieprintln!("Note".note() ": " note);
  // }
  // let mut lines = contents.lines();
  // let line_text = lines.nth(curr_line - 1).unwrap_or_else(|| exit(1));
  // if !line_text.trim().is_empty() {
  //   eprintln!();
  //   let mut padding = 0;
  //   for ch in line_text.chars() {
  //     match ch {
  //       '\t' => padding += 4,
  //       ' ' => padding += 1,
  //       _ => break,
  //     }
  //   }
  //   eprintln!("{}", &line_text[padding..]);
  //   eprintln!("{}{}", " ".repeat(curr_column - 1 - padding), "^".repeat(tok_len).red().bold());
  // }
  ieprintln!("  at " src_file ":" src_line ":" src_column);
  // eprintln!("  at {}:{}:{}", ctx.file, ctx.line, ctx.column);
  exit(1);
}

#[macro_export]
macro_rules! internal_err {
  ($($arg:expr)*) => {{
    use $crate::terminal::quit;
    quit("Internal error", &iformat!($($arg)*), file!(), line!(), column!());
  }};
}
#[macro_export]
macro_rules! os_err {
  ($($arg:expr)*) => {{
    use $crate::terminal::quit;
    quit("OS error", &iformat!($($arg)*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! argument_err {
  ($($arg:expr)*) => {{
    use $crate::terminal::quit;
    quit("Argument error", &iformat!($($arg)*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! syntax_err {
  ($($arg:expr)*) => {{
    use $crate::terminal::quit;
    quit("Syntax error", &iformat!($($arg)*), file!(), line!(), column!());
  }};
}
