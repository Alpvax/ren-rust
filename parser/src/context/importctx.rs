use ast::Import;

use crate::Token;

use super::{ContextNext, PartialParse, PartialResult};

#[derive(Debug, Clone, PartialEq)]
pub enum ImportContext {
    /// After "import" keyword
    Start,
    /// After import whitespace
    ExpectPath,
    /// After path parsed
    AfterPath(String),
    /// After "as" keyword
    StartAs(String),
    /// After as whitespace (first called with empty vec)
    ExpectNamespace(String, Vec<String>),
    /// After namespace, expect '.' or whitespace
    ExpectNSDelim(String, Vec<String>),
    /// After namespace parsed
    AfterAs(String, Vec<String>),
    /// After "exposing" keyword, expect whitespace or '{'
    StartExp(String, Vec<String>),
    /// Inside exposing block, expect name (first called with empty vec)
    ExpectExpName(String, Vec<String>, Vec<String>),
    /// After name, expect ',', '}' or whitespace
    ExpectExpDelim(String, Vec<String>, Vec<String>),
    //// After '}', no further tokens accepted
    // End(Import),
}

impl PartialParse for ImportContext {
    type Partial = Self;
    type Complete = Import;
    type Error = String;

    fn parse_next<'source>(self, token: Token, ctx: &mut ContextNext<'source>) -> PartialResult<Self, Import, String> {
        use crate::string::*;
        use ImportContext::*;
        use PartialResult::*;
        match (self, token) {
            (Start, Token::Whitespace) => Partial(ExpectPath),
            (ExpectPath, Token::SingleQuote) => {
                match ctx.parse_string(QuoteType::Single) {
                    Ok(s) => Partial(AfterPath(s)),
                    Result::Err(e) => Err(format!("Invalid path: Parse Err = {:?}", e)),
                }
            }
            (ExpectPath, lexer::Token::DoubleQuote) => {
                match ctx.parse_string(QuoteType::Double) {
                    Ok(s) => Partial(AfterPath(s)),
                    Result::Err(e) => Err(format!("Invalid path: Parse Err = {:?}", e)),
                }
            }
            (AfterPath(path), Token::Whitespace) => Partial(AfterPath(path)),
            (AfterPath(path), Token::KWAs) => Partial(StartAs(path)),
            (AfterPath(path), Token::KWExposing) => Partial(StartExp(path, Vec::new())),
            (AfterPath(path), tok) => todo!(
                "After path: {}; Recieved token {:?}; End import statement?!",
                path,
                tok
            ),
            (StartAs(path), Token::Whitespace) => Partial(ExpectNamespace(path, Vec::new())),
            (ExpectNamespace(path, mut ns), Token::Namespace(n)) => {
                ns.push(n);
                Partial(ExpectNSDelim(path, ns))
            }
            (ExpectNSDelim(path, ns), Token::Period) => Partial(ExpectNamespace(path, ns)),
            (ExpectNSDelim(path, ns), Token::Whitespace) => Partial(AfterAs(path, ns)),
            (AfterAs(path, ns), Token::KWExposing) => Partial(StartExp(path, ns)),
            (AfterAs(path, ns), tok) => todo!(
                "After path: {}, as: {:?}; Recieved token {:?}; End import statement?!",
                path,
                ns,
                tok,
            ),
            (StartExp(path, ns), Token::Whitespace) => Partial(StartExp(path, ns)),
            (StartExp(path, ns), Token::CurlyOpen) => Partial(ExpectExpName(path, ns, Vec::new())),
            (ExpectExpName(path, ns, exp), Token::Whitespace) => {
                Partial(ExpectExpName(path, ns, exp))
            }
            (ExpectExpName(path, ns, mut exp), Token::VarName(n)) => {
                exp.push(n);
                Partial(ExpectExpDelim(path, ns, exp))
            }
            (ExpectExpDelim(path, ns, exp), Token::Whitespace) => {
                Partial(ExpectExpDelim(path, ns, exp))
            }
            (ExpectExpDelim(path, ns, exp), Token::Comma) => Partial(ExpectExpName(path, ns, exp)),
            (ExpectExpDelim(path, name, bindings), Token::CurlyClose) => Complete(
                Import {
                    path,
                    name,
                    bindings,
                },
                None,
            ),
            (ctx, tok) => Err(format!("Unexpected token: {:?}; Context: {:?}", tok, ctx)),
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            ImportContext::AfterPath(_) /*| ImportContext::End*/ => true,
            ImportContext::ExpectNSDelim(_, ns) => ns.len() > 0,
            _ => false,
        }
    }
}
impl Default for ImportContext {
    fn default() -> Self {
        Self::Start
    }
}

#[cfg(test)]
mod import_tests {
    use super::*;

    // fn test_tokens(
    //     tokens: impl IntoIterator<Item = Token>,
    // ) -> PartialResult<ImportContext, Import, String> {
    //     use ImportContext::*;
    //     use PartialResult::*;

    //     tokens.into_iter().fold(Partial(Start), |res, token| {
    //         if let Partial(ctx) = res {
    //             ctx.parse_next(token)
    //         } else {
    //             res
    //         }
    //     })
    // }
    fn test_parse(input: &str) -> PartialResult<ImportContext, Import, String> {
        use ImportContext::*;
        use PartialResult::*;

        let mut lexer = lexer::lexer(input);
        let mut ctx_next = ContextNext { lexer: &mut lexer };

        // let mut ctx_next = Ctx { ctx: ContextNext { lexer: &mut lexer }, res: Partial(Start) };

        // lexer.fold(ctx_next.res, |res, token| {
        //     if let Partial(ctx) = res {
        //         ctx.parse_next(token)
        //     } else {
        //         res
        //     }
        // })

        let mut res = Partial(Start);
        loop {
            let token = ctx_next.next_token();
            if token.is_some() {
                res = if let Partial(ctx) = res {
                    ctx.parse_next(token.unwrap(), &mut ctx_next)
                } else {
                    return res;
                }
            } else {
                return res;
            }
        }
    }

    #[test]
    fn test_path_only() {
        let expected =
            PartialResult::Partial(ImportContext::AfterPath("some/path".to_owned()));
        // let single_res = test_tokens(vec![Token::Whitespace, Token::SingleQuote]);
        let single_res = test_parse(" 'some/path'");
        assert!(single_res.is_valid());
        assert_eq!(single_res, expected);

        // let double_res = test_tokens(vec![Token::Whitespace, Token::DoubleQuote]);
        let double_res = test_parse(r#" "some/path""#);
        assert!(double_res.is_valid());
        assert_eq!(double_res, expected);
    }

    #[test]
    fn test_namespaced_only() {
        let expected = PartialResult::Partial(ImportContext::ExpectNSDelim(
            "./some/path".to_owned(),
            vec!["Some".to_owned(), "Name".to_owned(), "Space".to_owned()],
        ));

        // let single_res = test_tokens(vec![
        //     Token::Whitespace,
        //     Token::SingleQuote,
        //     Token::Whitespace,
        //     Token::KWAs,
        //     Token::Whitespace,
        //     Token::Namespace("Some".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Name".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Space".to_owned()),
        // ]);
        let single_res = test_parse(" './some/path' as Some.Name.Space");
        assert!(single_res.is_valid());
        assert_eq!(single_res, expected);

        // let double_res = test_tokens(vec![
        //     Token::Whitespace,
        //     Token::DoubleQuote,
        //     Token::Whitespace,
        //     Token::KWAs,
        //     Token::Whitespace,
        //     Token::Namespace("Some".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Name".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Space".to_owned()),
        // ]);
        let double_res = test_parse(r#" "./some/path" as Some.Name.Space"#);
        assert!(double_res.is_valid());
        assert_eq!(double_res, expected);
    }

    #[test]
    fn test_exposing_only() {
        let expected = PartialResult::Complete(
            Import {
                path: "./some/path".to_owned(),
                name: Vec::new(),
                bindings: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
            },
            None,
        );

        // let single_res = test_tokens(vec![
        //     Token::Whitespace,
        //     Token::SingleQuote,
        //     Token::Whitespace,
        //     Token::KWExposing,
        //     Token::Whitespace, //Optional
        //     Token::CurlyOpen,
        //     Token::VarName("foo".to_owned()),
        //     Token::Comma,
        //     Token::Whitespace, //Optional
        //     Token::VarName("bar".to_owned()),
        //     Token::Whitespace, //Optional
        //     Token::Comma,
        //     Token::VarName("baz".to_owned()),
        //     Token::Whitespace, //Optional
        //     Token::CurlyClose,
        // ]);
        let single_res = test_parse(" './some/path' exposing {foo, bar ,baz }");
        assert!(single_res.is_valid());
        assert_eq!(single_res, expected);

        // let double_res = test_tokens(vec![
        //     Token::Whitespace,
        //     Token::DoubleQuote,
        //     Token::Whitespace,
        //     Token::KWExposing,
        //     Token::CurlyOpen,
        //     Token::Whitespace, //Optional
        //     Token::VarName("foo".to_owned()),
        //     Token::Comma,
        //     Token::VarName("bar".to_owned()),
        //     Token::Comma,
        //     Token::VarName("baz".to_owned()),
        //     Token::CurlyClose,
        // ]);
        let double_res = test_parse(r#" "./some/path" exposing{ foo,bar,baz}"#);
        assert!(double_res.is_valid());
        assert_eq!(double_res, expected);
    }

    #[test]
    fn test_namespaced_exposing() {
        let expected = PartialResult::Complete(
            Import {
                path: "./some/path".to_owned(),
                name: vec!["Some".to_owned(), "Name".to_owned(), "Space".to_owned()],
                bindings: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
            },
            None,
        );

        // let single_res = test_tokens(vec![
        //     Token::Whitespace,
        //     Token::SingleQuote,
        //     Token::Whitespace,
        //     Token::KWAs,
        //     Token::Whitespace,
        //     Token::Namespace("Some".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Name".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Space".to_owned()),
        //     Token::Whitespace,
        //     Token::KWExposing,
        //     Token::Whitespace, //Optional
        //     Token::CurlyOpen,
        //     Token::VarName("foo".to_owned()),
        //     Token::Comma,
        //     Token::Whitespace, //Optional
        //     Token::VarName("bar".to_owned()),
        //     Token::Whitespace, //Optional
        //     Token::Comma,
        //     Token::VarName("baz".to_owned()),
        //     Token::Whitespace, //Optional
        //     Token::CurlyClose,
        // ]);
        let single_res = test_parse(" './some/path' as Some.Name.Space exposing {foo, bar ,baz }");
        assert!(single_res.is_valid());
        assert_eq!(single_res, expected);

        // let double_res = test_tokens(vec![
        //     Token::Whitespace,
        //     Token::DoubleQuote,
        //     Token::Whitespace,
        //     Token::KWAs,
        //     Token::Whitespace,
        //     Token::Namespace("Some".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Name".to_owned()),
        //     Token::Period,
        //     Token::Namespace("Space".to_owned()),
        //     Token::Whitespace,
        //     Token::KWExposing,
        //     Token::CurlyOpen,
        //     Token::Whitespace, //Optional
        //     Token::VarName("foo".to_owned()),
        //     Token::Comma,
        //     Token::VarName("bar".to_owned()),
        //     Token::Comma,
        //     Token::VarName("baz".to_owned()),
        //     Token::CurlyClose,
        // ]);
        let double_res = test_parse(r#" "./some/path" as Some.Name.Space exposing{ foo,bar,baz}"#);
        assert!(double_res.is_valid());
        assert_eq!(double_res, expected);
    }
}
