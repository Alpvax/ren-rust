use std::path::Path;

use parser::parse_stmt_ast;
use rustyline::{error::ReadlineError, Editor};

mod command;
// mod config;
// mod env;
mod mode;

const COMMAND_START: &str = "//!";

pub fn init_repl<P>(history_file_path: &P) -> rustyline::Result<()>
where
    P: AsRef<Path> + ?Sized,
{
    let mut mode = mode::Modes::Rowan;
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(history_file_path).is_err() {
        println!("No previous history.");
    }
    println!("Ren language REPL. Version {ver} (Rust backend)\nUse {cmd} to send repl commands, e.g. {cmd}help", ver = "x.x.x", cmd = COMMAND_START);
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
                    if let Err(e) =
                        parse_stmt_ast(line.as_str()).and_then(|stmt| mode.handle_stmt(stmt))
                    {
                        println!("Error parsing line: {:?}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
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
