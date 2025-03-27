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
#[path = "context/context.rs"]
mod context;

use frontend::commands;
use context::Program;

fn main() {
  let mut program = Program::new();
  match program.command.as_ref() {
    "" | "help" => commands::help::run_command(),
    "version" => commands::version::run_command(),

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
    other => ieprintln!("Unknown command: " other),
  };
}
