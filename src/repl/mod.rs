use std::path::Path;

use parser::parse_stmt_ast;
use rustyline::{error::ReadlineError, Editor};

mod command;
// mod config;
// mod env;
mod mode;
pub use mode::Modes;
impl Default for Modes {
    fn default() -> Self {
        Self::Rowan
    }
}

const COMMAND_START: &str = "//!";

pub fn init_repl<P>(history_file_path: &P, start_mode: Modes) -> rustyline::Result<()>
where
    P: AsRef<Path> + ?Sized,
{
    let mut mode = start_mode;
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(history_file_path).is_err() {
        println!("No previous history.");
    }
    println!("Ren language REPL. Version {ver} (Rust backend)\nUse {cmd} to send repl commands, e.g. {cmd}help", ver = env!("CARGO_PKG_VERSION"), cmd = COMMAND_START);
    let mut out = std::io::stdout();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.starts_with(COMMAND_START) {
                    if let Err(e) = mode.handle_command(line[COMMAND_START.len()..].trim()) {
                        println!("{}", e);
                    }
                } else {
                    if let Err(e) = parse_stmt_ast(line.as_str()).and_then(|(stmt, line_lookup)| {
                        mode.handle_stmt(&mut out, stmt, &line_lookup)
                    }) {
                        println!("Error parsing line: {:?}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(history_file_path)
}
