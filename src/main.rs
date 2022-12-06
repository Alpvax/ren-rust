//mod ast;
//mod parser;
#[allow(dead_code)]
mod value;

mod repl;

mod cli;
#[cfg(test)]
mod test;

use repl::Modes as ReplModes;

#[cfg(feature = "reqwest")]
fn parse_remote_file(url: &str) -> parser::Parsed {
    use reqwest;
    reqwest::blocking::get(url)
        .and_then(|r| r.text())
        .map(|input| parser::parse_module(input.as_str()))
}

enum CliError {
    Repl(::rustyline::error::ReadlineError),
    Parse(::clap::Error),
    Io(std::io::Error),
    Stmt(&'static str),
}
impl From<::rustyline::error::ReadlineError> for CliError {
    fn from(e: ::rustyline::error::ReadlineError) -> Self {
        Self::Repl(e)
    }
}
impl From<::clap::Error> for CliError {
    fn from(e: ::clap::Error) -> Self {
        Self::Parse(e)
    }
}
impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<&'static str> for CliError {
    fn from(e: &'static str) -> Self {
        Self::Stmt(e)
    }
}
impl core::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Repl(e) => e.fmt(f),
            CliError::Parse(e) => e.fmt(f),
            CliError::Io(e) => e.fmt(f),
            CliError::Stmt(s) => s.fmt(f),
        }
    }
}
impl core::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as core::fmt::Display>::fmt(&self, f)
        // match self {
        //     CliError::Repl(e) => e.fmt(f),
        //     CliError::Io(e) => e.fmt(f),
        //     CliError::Stmt(s) => s.fmt(f),
        // }
    }
}
impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CliError::Repl(e) => Some(e),
            CliError::Io(e) => Some(e),
            _ => None,
        }
    }
}

fn main() -> Result<(), CliError> {
    let args = cli::parse();
    match args.cmd {
        cli::Cmd::Repl { mode, histfile } => repl::init_repl(&histfile, mode)?,
        cli::Cmd::Parse {
            infile,
            ofile,
            format,
            stdinput,
        } => {
            let input = if let Some(ipath) = infile {
                std::fs::read_to_string(ipath)?
            } else {
                stdinput.unwrap()
            };
            let mut output = Vec::new();
            parser::parse_stmt_ast(&input)
                .and_then(|(s, ll)| format.handle_stmt(&mut output, s, &ll))?;
            if let Some(opath) = ofile {
                std::fs::write(opath, output)?
            }
        }
    };
    Ok(())
    //println!("{:?}", parser::parse(SAMPLE));
}
