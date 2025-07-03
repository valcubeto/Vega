use std::path::{ Path, PathBuf };
use std::env;
use crate::{ some, ok };

pub struct Program {
  /// Path("/home")
  pub cwd: Box<Path>,
  /// Path("/usr/bin/vega")
  pub this: Box<Path>,
  /// "run"
  pub command: Box<str>,
  /// "main"
  pub input: Box<str>,
  /// 0
  pub index: u32,
}

pub struct Position {
  pub line: usize,
  pub column: usize,
}

pub struct Flag<'a> {
  pub long: &'a str,
  pub short: char,
  pub take_value: bool
}

impl Program {
  pub fn new() -> Self {
    let flags = [
      ("c", "count"),
      ("p", "plain"),
      ("d", "debug"),
      ("q", "quiet"),
      ("w", "workspace"),
    ];
    let mut argv = env::args();
    let this = some!(argv.next() => argument_err!("argv is empty!"));
    let this = PathBuf::from(this).into_boxed_path();
    let command = argv.next().unwrap_or("".into()).into_boxed_str();
    // let args = ;
    // let flags = ;
    Self {
      cwd: ok!(env::current_dir() => os_err!("cannot get the current directory")).into_boxed_path(),
      this,
      command,
      input: Box::from(""),
      index: 0
    }
  }
  /// Get the current file position based on the index.
  pub fn pos(&self) -> Position {
    todo!()
  }
}
