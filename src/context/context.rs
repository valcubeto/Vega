use std::path::Path;
use std::env;

pub struct Program {
  pub cwd: Box<Path>,
  pub command: Box<str>,
}

impl Program {
  pub fn new() -> Self {
    // let mut argv = env::args();
    // let this = expect_some!(argv.next() => "argv is empty!");
    // let command = ;
    // let args = ;
    // let flags = ;
    Self {
      cwd: env::current_dir().unwrap().into_boxed_path(),
      command: env::args().nth(1).unwrap().into_boxed_str(),
    }
  }
}
