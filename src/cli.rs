use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author = "Alpvax", version, about)]
/// A custom Ren language parser, written from the ground up in rust
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) cmd: Cmd,
    // #[clap(short, long, action = clap::ArgAction::Count)]
    // /// How detailed the messages to be output are
    // verbosity: u8,
}

#[derive(Debug, Clone, Subcommand)]
pub(crate) enum Cmd {
    Repl {
        #[clap(short, long, default_value_t)]
        /// The mode to start the REPL in
        mode: crate::ReplModes,
        #[clap(short = 'H', long, default_value = "./.repl_history")]
        /// The location of the file to use for saving/loading the REPL history
        histfile: PathBuf,
    },
    Parse {
        #[clap(short, long)]
        /// The file to convert. If missing, will read from STDIN
        infile: Option<PathBuf>,
        #[clap(short, long)]
        /// The file to write the output to. If missing, will write to STDOUT
        ofile: Option<PathBuf>,
        #[clap(short, long, default_value_t)]
        /// The format of the output
        format: crate::ReplModes, //TODO: Better output format options, seperate from repl modes
        #[clap(hide = true)]
        stdinput: Option<String>,
    }
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}