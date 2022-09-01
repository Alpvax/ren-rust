pub(super) trait ReplCommand {
    fn ident() -> &'static str;
    /// Returns a tuple of (args, description)
    fn description() -> (&'static str, &'static str);
    fn handle_command(cmd: &str, _todo_repl_mutator: ()) -> Result<(), &str>;
}
