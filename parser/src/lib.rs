pub(crate) mod context;
pub(crate) mod string;

pub(crate) type Token = lexer::Token;

use ast::Import;

use crate::context::{PartialResult, PartialParse};

pub fn parse_import<'source>(source: &'source str) -> Result<ast::Import, String> {
    let mut lexer = lexer::lexer(source);
    if let Some(Token::KWImport) = lexer.next() {
        let mut ctx_next = context::ContextNext { lexer: &mut lexer };
        let mut res = PartialResult::Partial(context::ImportContext::Start);
        loop {
            let token = ctx_next.next_token();
            if token.is_some() {
                res = if let PartialResult::Partial(ctx) = res {
                    ctx.parse_next(token.unwrap(), &mut ctx_next)
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        match res {
            PartialResult::Partial(ctx) => {
                match ctx {
                    context::ImportContext::AfterPath(path) => Ok(Import {
                        path,
                        name: Vec::new(),
                        bindings: Vec::new(),
                    }),
                    context::ImportContext::ExpectNSDelim(path, name) => Ok(Import {
                        path,
                        name,
                        bindings: Vec::new(),
                    }),
                    _ => Err(format!("Incomplete import statement! {:?}", ctx)),
                }
            },
            PartialResult::Complete(i, tok) => Ok(i),
            PartialResult::Err(e) => Err(e),
        }
    } else {
        Err("Text must start with \"import\"!".to_owned())
    }
}

#[test]
fn parse_test_import() {
    assert_eq!(parse_import("import 'some/thing' as Foo.Thing exposing { foo, bar, baz }"), Ok(Import {
        path: "some/thing".to_owned(),
        name: vec!["Foo".to_owned(), "Thing".to_owned()],
        bindings: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
    }));
}
