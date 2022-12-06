use parser::{
    lower_ast::{Decl, Expr, Import, ToHIR},
    REPLStmt,
};

pub(super) trait ReplMode {
    fn name() -> &'static str;
    fn handle_command<'c, 'r>(cmd: &'c str, mode: &'r mut Modes) -> Result<(), &'c str>;
    fn handle_stmt<W: std::io::Write>(
        w: W,
        stmt: REPLStmt<Decl, Expr, Import>,
        line_lookup: &::line_col::LineColLookup,
    ) -> Result<(), &'static str>;
}

macro_rules! make_modes {
    ($($name:ident {
        $(name: )? $display:literal,
        commands: {
            $($cmd:ty),* $(,)?
        }$(,)?
        $(handle_stmt: )? $(stmt =>)? {
            $(REPLStmt::)?Decl($decl_id:ident $(,$d_l_lookup:ident)?) => $decl_body:expr,
            $(REPLStmt::)?Expr($expr_id:ident $(,$e_l_lookup:ident)?) => $expr_body:expr,
            $(REPLStmt::)?Import($imp_id:ident $(,$i_l_lookup:ident)?) => $imp_body:expr,
        }
    }$(,)?)+) => {
        #[derive(Clone, Copy, Debug)]
        pub enum Modes {
            $($name,)+
        }
        #[cfg(feature = "cli")]
        impl ::clap::ValueEnum for Modes {
            fn value_variants<'a>() -> &'a [Self] {
                &[$(Self::$name),+]
            }
        
            fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
                Some(match self {
                    $(Self::$name => clap::builder::PossibleValue::new(stringify!($name)).alias($display).help($display),)+
                })
            }
        }
        impl core::fmt::Display for Modes {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(Self::$name => $display,)+
                })
            }
        }
        impl Modes {
            fn switch_mode(&mut self, mode_name: &str) -> bool {
                match mode_name {
                    $($display => *self = Self::$name,)+
                    _ => return false,
                }
                return true;
            }
            pub fn handle_command<'r, 'c>(&'r mut self, cmd: &'c str) -> Result<(), &'c str> {
                match &self {
                    $(Self::$name => $name::handle_command(cmd, self),)+
                }
            }
            pub fn handle_stmt<W: std::io::Write>(&self, w: W, stmt: REPLStmt<Decl, Expr, Import>, line_lookup: &::line_col::LineColLookup) -> Result<(), &'static str> {
                match &self {
                    $(Self::$name => $name::handle_stmt(w, stmt, line_lookup),)+
                }
            }
        }
        $(
            #[derive(Debug, Clone, Copy)]
            pub(super) struct $name;
            impl ReplMode for $name {
                fn name() -> &'static str {
                    $display
                }
                fn handle_command<'c, 'r>(cmd: &'c str, mode: &'r mut Modes) -> Result<(), &'c str> {
                    #[allow(unused_imports)]
                    use crate::repl::command::ReplCommand;
                    if "help".starts_with(cmd) {
                        println!(r"Commands:
                            help                Display this message
                            mode [mode_name]    Print the current mode or switch mode
                        ")
                        $({
                            let (args, desc) = <$cmd as ReplCommand>::description();
                            println!("\t{} {}\t\t\t\t{}", <$cmd as ReplCommand>::ident(), args, description);
                        })*
                    }
                    $({
                        type Cmd = <$cmd as ReplCommand>;
                        let ident = Cmd::ident()
                        if cmd.starts_with(ident) {
                            return Cmd::handle_command(cmd[ident.len()..].trim(), ());
                        }
                    })*

                    if cmd.starts_with("mode") {
                        if cmd.len() <= 4 {
                            println!("Current mode: {}", $display);
                            return Ok(());
                        } else if mode.switch_mode(&cmd[4..].trim()) {
                            return Ok(());
                        }
                    }

                    Err("Unknown command")
                }
                fn handle_stmt<W: std::io::Write>(#[allow(unused_variables)]w: W, stmt: REPLStmt<Decl, Expr, Import>, line_lookup: &::line_col::LineColLookup) -> Result<(), &'static str> {
                    match (stmt, line_lookup, ()) {
                        (REPLStmt::Decl($decl_id), $($d_l_lookup,)? ..) => $decl_body,
                        (REPLStmt::Expr($expr_id), $($e_l_lookup,)? ..) => $expr_body,
                        (REPLStmt::Import($imp_id), $($i_l_lookup,)? ..) => $imp_body,
                        _ => Ok(()),
                    }
                }
            }
        )+
    };
}

make_modes! {
    Rowan {
        "rowan AST",
        commands: {},
        {
            Decl(decl) => Ok(println!("{:#?}", decl)),
            Expr(expr) => Ok(println!("{:#?}", expr)),
            Import(imp) => Ok(println!("{:#?}", imp)),
        }
    }
    Higher {
        "higher AST",
        commands: {},
        {
            Decl(decl, line_lookup) => Ok(println!("{:?}", decl.to_higher_ast(line_lookup))),
            Expr(expr, line_lookup) => Ok(println!("{:?}", expr.to_higher_ast(line_lookup))),
            Import(imp, line_lookup) => Ok(println!("{:?}", imp.to_higher_ast(line_lookup))),
        }
    }
    Json {
        "json",
        commands: {},
        {
            Decl(decl, line_lookup) => {
                if let Err(e) = decl.to_higher_ast(line_lookup).to_json_writer(std::io::stdout(), false) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Expr(expr, line_lookup) => {
                if let Err(e) = expr.to_higher_ast(line_lookup).to_json_writer(std::io::stdout(), false) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Import(imp, line_lookup) => {
                if let Err(e) = imp.to_higher_ast(line_lookup).to_json_writer(std::io::stdout(), false) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
        }
    }
    JsonPretty {
        "json (pretty)",
        commands: {},
        {
            Decl(decl, line_lookup) => {
                if let Err(e) = decl.to_higher_ast(line_lookup).to_json_writer(std::io::stdout(), true) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Expr(expr, line_lookup) => {
                if let Err(e) = expr.to_higher_ast(line_lookup).to_json_writer(std::io::stdout(), true) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Import(imp, line_lookup) => {
                if let Err(e) = imp.to_higher_ast(line_lookup).to_json_writer(std::io::stdout(), true) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
        }
    }
}
// pub(super) struct Rowan;
// impl ReplMode for Rowan {
//     fn name() -> &'static str {
//         "rowan AST"
//     }

//     fn handle_command(cmd: &str) -> Result<(), &'static str> {
//         if &cmd[0..5] == "mode " {

//         }
//     }

//     fn handle_stmt<D, E, I>(stmt: REPLStmt<D, E, I>) -> Result<(), &'static str> {
//         todo!()
//     }
// }
