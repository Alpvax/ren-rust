use crate::parse_module;
use expect_test::expect;

fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse_module(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

mod imports {
    use super::*;

    #[test]
    fn parse_path_only() {
        check(
            r#"import "path""#,
            expect![[r#"
                Context(Module)@0..13
                  Context(Imports)@0..13
                    Context(Import)@0..13
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(SymDoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\"""#]],
        )
    }

    #[test]
    fn parse_as() {
        check(
            r#"import "path" as Name"#,
            expect![[r#"
                Context(Module)@0..21
                  Context(Imports)@0..21
                    Context(Import)@0..21
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(SymDoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(IdUpper)@17..21
                        Token(IdUpper)@17..21 "Name""#]],
        )
    }

    #[test]
    fn parse_as_multi() {
        check(
            r#"import "path" as Name.Space"#,
            expect![[r#"
                Context(Module)@0..27
                  Context(Imports)@0..27
                    Context(Import)@0..27
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(SymDoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(IdUpper)@17..27
                        Token(IdUpper)@17..21 "Name"
                        Token(SymDot)@21..22 "."
                        Token(IdUpper)@22..27 "Space""#]],
        )
    }

    // #[test]
    // fn parse_exposing() {
    //     check(
    //         r#"import "path" exposing {foo}"#,
    //         expect![[r#"
    //             Context(Module)@0..28
    //               Context(Imports)@0..28
    //                 Context(Import)@0..28
    //                   Token(KWImport)@0..6 "import"
    //                   Token(Whitespace)@6..7 " "
    //                   Context(String)@7..13
    //                     Token(SymDoubleQuote)@7..8 "\""
    //                     StringToken(Text)@8..12 "path"
    //                     StringToken(Delimiter)@12..13 "\""
    //                   Token(Whitespace)@13..14 " "
    //                   Token(KWExposing)@14..22 "exposing"
    //                   Token(Whitespace)@22..23 " "
    //                   Token(SymLBrace)@23..24 "{"
    //                   Context(ExposingBlock)@24..28
    //                     Token(IdLower)@24..27 "foo"
    //                     Token(SymRBrace)@27..28 "}""#]],
    //     )
    // }

    // #[test]
    // fn parse_exposing_multi() {
    //     check(
    //         r#"import "path" exposing {foo, bar}"#,
    //         expect![[r#"
    //             Context(Module)@0..33
    //               Context(Imports)@0..33
    //                 Context(Import)@0..33
    //                   Token(KWImport)@0..6 "import"
    //                   Token(Whitespace)@6..7 " "
    //                   Context(String)@7..13
    //                     Token(SymDoubleQuote)@7..8 "\""
    //                     StringToken(Text)@8..12 "path"
    //                     StringToken(Delimiter)@12..13 "\""
    //                   Token(Whitespace)@13..14 " "
    //                   Token(KWExposing)@14..22 "exposing"
    //                   Token(Whitespace)@22..23 " "
    //                   Token(SymLBrace)@23..24 "{"
    //                   Context(ExposingBlock)@24..33
    //                     Token(IdLower)@24..27 "foo"
    //                     Token(Comma)@27..28 ","
    //                     Token(Whitespace)@28..29 " "
    //                     Token(IdLower)@29..32 "bar"
    //                     Token(SymRBrace)@32..33 "}""#]],
    //     )
    // }

    #[test]
    fn parse_full() {
        check(
            r#"import "path" as Name.Space"#,
            expect![[r#"
                Context(Module)@0..27
                  Context(Imports)@0..27
                    Context(Import)@0..27
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(SymDoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(IdUpper)@17..27
                        Token(IdUpper)@17..21 "Name"
                        Token(SymDot)@21..22 "."
                        Token(IdUpper)@22..27 "Space""#]],
        )
    }

    #[test]
    fn multiple() {
        check(
            r#"import "path" as Name.Space
        import "./path2""#,
            expect![[r#"
                Context(Module)@0..52
                  Context(Imports)@0..52
                    Context(Import)@0..36
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(SymDoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(IdUpper)@17..36
                        Token(IdUpper)@17..21 "Name"
                        Token(SymDot)@21..22 "."
                        Token(IdUpper)@22..27 "Space"
                        Token(Whitespace)@27..36 "\n        "
                    Context(Import)@36..52
                      Token(KWImport)@36..42 "import"
                      Token(Whitespace)@42..43 " "
                      Context(String)@43..52
                        Token(SymDoubleQuote)@43..44 "\""
                        StringToken(Text)@44..51 "./path2"
                        StringToken(Delimiter)@51..52 "\"""#]],
        )
    }
}

mod declaration {
    use super::*;

    #[test]
    fn ext() {
        check(
            r#"ext var = "name""#,
            expect![[r#"
                Context(Module)@0..16
                  Context(Declarations)@0..16
                    Context(Declaration)@0..16
                      Token(KWExt)@0..3 "ext"
                      Token(Whitespace)@3..4 " "
                      Token(IdLower)@4..7 "var"
                      Token(Whitespace)@7..8 " "
                      Token(SymEquals)@8..9 "="
                      Token(Whitespace)@9..10 " "
                      Context(String)@10..16
                        Token(SymDoubleQuote)@10..11 "\""
                        StringToken(Text)@11..15 "name"
                        StringToken(Delimiter)@15..16 "\"""#]],
        );
    }

    #[test]
    fn public_ext() {
        check(
            r#"pub ext var = "name""#,
            expect![[r#"
                Context(Module)@0..20
                  Context(Declarations)@0..20
                    Context(Declaration)@0..20
                      Token(KWPub)@0..3 "pub"
                      Token(Whitespace)@3..4 " "
                      Token(KWExt)@4..7 "ext"
                      Token(Whitespace)@7..8 " "
                      Token(IdLower)@8..11 "var"
                      Token(Whitespace)@11..12 " "
                      Token(SymEquals)@12..13 "="
                      Token(Whitespace)@13..14 " "
                      Context(String)@14..20
                        Token(SymDoubleQuote)@14..15 "\""
                        StringToken(Text)@15..19 "name"
                        StringToken(Delimiter)@19..20 "\"""#]],
        );
    }

    #[test]
    fn local() {
        check(
            "let var = 3",
            expect![[r#"
                Context(Module)@0..11
                  Context(Declarations)@0..11
                    Context(Declaration)@0..11
                      Token(KWLet)@0..3 "let"
                      Token(Whitespace)@3..4 " "
                      Token(IdLower)@4..7 "var"
                      Token(Whitespace)@7..8 " "
                      Token(SymEquals)@8..9 "="
                      Context(Expr)@9..11
                        Token(Whitespace)@9..10 " "
                        Token(Number)@10..11 "3""#]],
        );
    }

    #[test]
    fn public() {
        check(
            r"pub let var = foo",
            expect![[r#"
                Context(Module)@0..17
                  Context(Declarations)@0..17
                    Context(Declaration)@0..17
                      Token(KWPub)@0..3 "pub"
                      Token(Whitespace)@3..4 " "
                      Token(KWLet)@4..7 "let"
                      Token(Whitespace)@7..8 " "
                      Token(IdLower)@8..11 "var"
                      Token(Whitespace)@11..12 " "
                      Token(SymEquals)@12..13 "="
                      Context(Expr)@13..17
                        Token(Whitespace)@13..14 " "
                        Token(IdLower)@14..17 "foo""#]],
        );
    }
}
