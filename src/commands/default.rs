//! Default command

use term_lab::styles::Stylize;

pub fn run_command() {
  iprintln!("Welcome to Vega!");
  iprintln!("Running " {crate::strings::VERSION.italics()});
  iprintln!("Run " {"vega help".italics()});
}
