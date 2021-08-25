pub(crate) use importctx::ImportContext;
pub(crate) use partial::*;

mod importctx;
mod partial;
pub(crate) mod stack;
pub(crate) mod string;

#[allow(dead_code)] //XXX
#[derive(Debug)]
pub enum Context {
    // Module toplevel
    Imports,
    Declarations,

    // Imports
    Import(ImportContext),

    // Declaration
    Public,

    Function,
    Variable,
    Enum,

    // Expression
    Expression(ExpressionContext),
}

#[allow(dead_code)] //XXX
#[derive(Debug)]
pub enum ExpressionContext {
    Curly,
    /// Expression block
    BindingBlock,
    ObjectLit,

    ArrayLit,

    StringLit(string::StringType),

    Template,
    TemplateExpr,
}

impl Context {
    // pub fn begin_context<'s, T: lexer::Logos<'s>>(
    //     &mut self,
    //     parent: Option<Context>,
    //     lexer: lexer::Lexer<'s, T>,
    // ) {
    //     if let Context::Expression(ExpressionContext::StringLit(t)) = self {
    //         lexer.morph()
    //     }
    // }
}

// #[derive(Debug)]
// pub enum ModuleContext {
//     Imports(Vec<ast::Import>, Option<ImportContext>),
//     Declarations(Vec<(ast::Visibility, ast::Declaration)>),
// }

// pub enum ImportContext {
//     /// After "import" keyword
//     Start,
//     /// After parsing path
//     ImportPath(String),
//     /// After path and "as" keyword, inside namespace
//     ImportAs(String, Vec<String>),
//     /// After path and optional namespace, inside "exposing" block
//     ImportExposing(String, Vec<String>),
// }
