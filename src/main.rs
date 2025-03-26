#[macro_use]
extern crate ifmt;

#[macro_use]
#[path = "terminal/terminal.rs"]
mod terminal;
mod strings;
#[path = "frontend/frontend.rs"]
mod frontend;
// mod backend;
mod globals;

use frontend::{ context::Program, commands };

fn main() {
  let mut program = Program::new();
  match program.command.as_ref() {
    "version" => commands::version::run_command(),
    "help" => todo!(),

    "init" => todo!(),
    "new" => todo!(),
    "add" => todo!(),
    "remove" => todo!(),
    "update" => todo!(),
    "clean" => todo!(),

    "eval" => todo!(),
    "print" => todo!(),
    "repl" => todo!(),
    "run" => todo!(),
    "build" => todo!(),
    "test" => todo!(),
    other => ieprintln!("Unknown command \"" other;? "\"")
  }
}
