use std::path::{Path, PathBuf};
use std::env;

pub struct Program {
  /// /home
  pub cwd: Box<Path>,
  /// /bin/vega
  pub this: Box<Path>,
  /// run
  pub command: Box<str>,
  /// main.sny
  pub input: Option<Box<Path>>,
  /// 0
  pub index: u32,
}

pub struct Position {
  pub line: usize,
  pub column: usize,
}

impl Program {
  pub fn new() -> Self {
    let flags = [
      ("-n", "--no-flood"),
      ("-p", "--plain"),
      ("-d", "--debug"),
      ("-q", "--quiet"),
      ("-w", "--workspace"),
    ];
    let mut argv = env::args();
    let this = crate::expect!(argv.next() => argument_err!("argv is empty!"));
    let this = PathBuf::from(this).into_boxed_path();
    let command = argv.next().unwrap_or("".into()).into_boxed_str();
    // let args = ;
    // let flags = ;
    Self {
      cwd: env::current_dir().unwrap().into_boxed_path(),
      this,
      command,
      input: None,
      index: 0
    }
  }
  /// Get the current file position based on the index.
  pub fn pos(&self) -> Position {
    todo!()
  }
}
