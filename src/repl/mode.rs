use parser::{
    lower_ast::{Decl, Expr, Import, ToHIR},
    REPLStmt,
};

pub(super) trait ReplMode {
    fn name() -> &'static str;
    fn handle_command<'c, 'r>(cmd: &'c str, mode: &'r mut Modes) -> Result<(), &'c str>;
    fn handle_stmt(stmt: REPLStmt<Decl, Expr, Import>) -> Result<(), &'static str>;
}

macro_rules! make_modes {
    ($($name:ident {
        $(name: )? $display:literal,
        commands: {
            $($cmd:ty),* $(,)?
        }$(,)?
        $(handle_stmt: )? $(stmt =>)? {
            $(REPLStmt::)?Decl($decl_id:ident) => $decl_body:expr,
            $(REPLStmt::)?Expr($expr_id:ident) => $expr_body:expr,
            $(REPLStmt::)?Import($imp_id:ident) => $imp_body:expr,
        }
    }$(,)?)+) => {
        #[derive(Clone, Copy, Debug)]
        pub(super) enum Modes {
            $($name,)+
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
            pub fn handle_stmt(&self, stmt: REPLStmt<Decl, Expr, Import>) -> Result<(), &'static str> {
                match &self {
                    $(Self::$name => $name::handle_stmt(stmt),)+
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
                fn handle_stmt(stmt: REPLStmt<Decl, Expr, Import>) -> Result<(), &'static str> {
                    match stmt {
                        REPLStmt::Decl($decl_id) => $decl_body,
                        REPLStmt::Expr($expr_id) => $expr_body,
                        REPLStmt::Import($imp_id) => $imp_body,
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
            Decl(decl) => Ok(println!("{:?}", decl.to_higher_ast())),
            Expr(expr) => Ok(println!("{:?}", expr.to_higher_ast())),
            Import(imp) => Ok(println!("{:?}", imp.to_higher_ast())),
        }
    }
    Json {
        "json",
        commands: {},
        {
            Decl(decl) => {
                if let Err(e) = decl.to_higher_ast().to_json_writer(std::io::stdout(), false) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Expr(expr) => {
                if let Err(e) = expr.to_higher_ast().to_json_writer(std::io::stdout(), false).map_err(|e| format!("{}", e)) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Import(imp) => {
                if let Err(e) = imp.to_higher_ast().to_json_writer(std::io::stdout(), false).map_err(|e| format!("{}", e)) {
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
            Decl(decl) => {
                if let Err(e) = decl.to_higher_ast().to_json_writer(std::io::stdout(), true) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Expr(expr) => {
                if let Err(e) = expr.to_higher_ast().to_json_writer(std::io::stdout(), true).map_err(|e| format!("{}", e)) {
                    println!("{}", e);
                }
                print!("\n"); // Force flush
                Ok(())
            },
            Import(imp) => {
                if let Err(e) = imp.to_higher_ast().to_json_writer(std::io::stdout(), true).map_err(|e| format!("{}", e)) {
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
