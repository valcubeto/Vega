use std::path::Path;
use std::env;

pub struct Program {
  /// /home
  pub cwd: Box<Path>,
  /// /bin/vega
  pub this: Box<Path>,
  /// run
  pub command: Box<str>,
  /// main.sny
  pub file: Box<Path>,
  /// 0
  pub index: u32,
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
    let this = expect!(argv.next() => "argv is empty!");
    // let command = ;
    // let args = ;
    // let flags = ;
    Self {
      cwd: env::current_dir().unwrap().into_boxed_path(),
      command: env::args().nth(1).unwrap().into_boxed_str(),
    }
  }
  pub fn pos(&self) -> (usize, usize) {
    todo!()
  }
}
