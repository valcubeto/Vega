#![macro_use]

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
#[path = "commands/commands.rs"]
mod commands;
mod macros;

use context::Program;

fn main() {
  let mut program = Program::new();
  todo!();
  match program.command.as_ref() {
    // These should be loaded anyways
    "--version" => commands::version::run_command(),

    // Bad approach: this loads ALL the commands, even if they are not used.
    // TODO: change to an external-command-based approach, like Cargo does.
    "" => commands::default::run_command(),
    "help" => commands::help::run_command(),
    // "init" => commands::init::run_command(),
    "create" => commands::create::run_command(&mut program),
    // "add" => commands::add::run_command(),
    // "remove" => commands::remove::run_command(),
    // "update" => commands::update::run_command(),
    // "clean" => commands::clean::run_command(),

    // "build" => commands::build::run_command(),
    "run" => commands::run::run_command(&mut program),
    // "eval" => commands::eval::run_command(),
    // "print" => commands::print::run_command(),
    // "test" => commands::test::run_command(),
    // "repl" => commands::repl::run_command(),

    other => ieprintln!("Unknown command: " other),
  };
}
