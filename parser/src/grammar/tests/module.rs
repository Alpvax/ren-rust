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
                        Token(DoubleQuote)@7..8 "\""
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
                        Token(DoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(NameSpace)@17..21
                        Token(Namespace)@17..21 "Name""#]],
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
                        Token(DoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(NameSpace)@17..27
                        Token(Namespace)@17..21 "Name"
                        Token(Period)@21..22 "."
                        Token(Namespace)@22..27 "Space""#]],
        )
    }

    #[test]
    fn parse_exposing() {
        check(
            r#"import "path" exposing {foo}"#,
            expect![[r#"
                Context(Module)@0..28
                  Context(Imports)@0..28
                    Context(Import)@0..28
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(DoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWExposing)@14..22 "exposing"
                      Token(Whitespace)@22..23 " "
                      Token(CurlyOpen)@23..24 "{"
                      Context(ExposingBlock)@24..28
                        Token(VarName)@24..27 "foo"
                        Token(CurlyClose)@27..28 "}""#]],
        )
    }

    #[test]
    fn parse_exposing_multi() {
        check(
            r#"import "path" exposing {foo, bar}"#,
            expect![[r#"
                Context(Module)@0..33
                  Context(Imports)@0..33
                    Context(Import)@0..33
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(DoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWExposing)@14..22 "exposing"
                      Token(Whitespace)@22..23 " "
                      Token(CurlyOpen)@23..24 "{"
                      Context(ExposingBlock)@24..33
                        Token(VarName)@24..27 "foo"
                        Token(Comma)@27..28 ","
                        Token(Whitespace)@28..29 " "
                        Token(VarName)@29..32 "bar"
                        Token(CurlyClose)@32..33 "}""#]],
        )
    }

    #[test]
    fn parse_full() {
        check(
            r#"import "path" as Name.Space exposing {foo, bar}"#,
            expect![[r#"
                Context(Module)@0..47
                  Context(Imports)@0..47
                    Context(Import)@0..47
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(DoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(NameSpace)@17..28
                        Token(Namespace)@17..21 "Name"
                        Token(Period)@21..22 "."
                        Token(Namespace)@22..27 "Space"
                        Token(Whitespace)@27..28 " "
                      Token(KWExposing)@28..36 "exposing"
                      Token(Whitespace)@36..37 " "
                      Token(CurlyOpen)@37..38 "{"
                      Context(ExposingBlock)@38..47
                        Token(VarName)@38..41 "foo"
                        Token(Comma)@41..42 ","
                        Token(Whitespace)@42..43 " "
                        Token(VarName)@43..46 "bar"
                        Token(CurlyClose)@46..47 "}""#]],
        )
    }

    #[test]
    fn multiple() {
        check(
            r#"import "path" as Name.Space
        import "./path2" exposing {foo, bar}"#,
            expect![[r#"
                Context(Module)@0..72
                  Context(Imports)@0..72
                    Context(Import)@0..36
                      Token(KWImport)@0..6 "import"
                      Token(Whitespace)@6..7 " "
                      Context(String)@7..13
                        Token(DoubleQuote)@7..8 "\""
                        StringToken(Text)@8..12 "path"
                        StringToken(Delimiter)@12..13 "\""
                      Token(Whitespace)@13..14 " "
                      Token(KWAs)@14..16 "as"
                      Token(Whitespace)@16..17 " "
                      Context(NameSpace)@17..36
                        Token(Namespace)@17..21 "Name"
                        Token(Period)@21..22 "."
                        Token(Namespace)@22..27 "Space"
                        Token(Whitespace)@27..36 "\n        "
                    Context(Import)@36..72
                      Token(KWImport)@36..42 "import"
                      Token(Whitespace)@42..43 " "
                      Context(String)@43..52
                        Token(DoubleQuote)@43..44 "\""
                        StringToken(Text)@44..51 "./path2"
                        StringToken(Delimiter)@51..52 "\""
                      Token(Whitespace)@52..53 " "
                      Token(KWExposing)@53..61 "exposing"
                      Token(Whitespace)@61..62 " "
                      Token(CurlyOpen)@62..63 "{"
                      Context(ExposingBlock)@63..72
                        Token(VarName)@63..66 "foo"
                        Token(Comma)@66..67 ","
                        Token(Whitespace)@67..68 " "
                        Token(VarName)@68..71 "bar"
                        Token(CurlyClose)@71..72 "}""#]],
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
                      Token(VarName)@4..7 "var"
                      Token(Whitespace)@7..8 " "
                      Token(OpAssign)@8..9 "="
                      Token(Whitespace)@9..10 " "
                      Context(String)@10..16
                        Token(DoubleQuote)@10..11 "\""
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
                      Token(VarName)@8..11 "var"
                      Token(Whitespace)@11..12 " "
                      Token(OpAssign)@12..13 "="
                      Token(Whitespace)@13..14 " "
                      Context(String)@14..20
                        Token(DoubleQuote)@14..15 "\""
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
                      Token(VarName)@4..7 "var"
                      Token(Whitespace)@7..8 " "
                      Token(OpAssign)@8..9 "="
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
                      Token(VarName)@8..11 "var"
                      Token(Whitespace)@11..12 " "
                      Token(OpAssign)@12..13 "="
                      Context(Expr)@13..17
                        Token(Whitespace)@13..14 " "
                        Token(VarName)@14..17 "foo""#]],
        );
    }
}
