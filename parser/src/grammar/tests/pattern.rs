use expect_test::expect;

fn check(input: &str, expected_tree: expect_test::Expect) {
    let mut p = crate::parser::Parser::new(input);
    super::super::pattern::parse_pattern(&mut p);
    let parse = p.parse();
    expected_tree.assert_eq(&parse.debug_tree());
}

#[test]
fn parse_any() {
    check(
        "_",
        expect![[r#"
        Context(Pattern)@0..1
          Token(SymUnderscore)@0..1 "_""#]],
    );
}

#[test]
fn parse_var() {
    check(
        "varName1",
        expect![[r#"
        Context(Pattern)@0..8
          Token(IdLower)@0..8 "varName1""#]],
    )
}

#[test]
fn parse_constructor() {
    check(
        "#foo bar 3",
        expect![[r##"
            Context(Pattern)@0..10
              Context(Constructor)@0..10
                Token(SymHash)@0..1 "#"
                Token(IdLower)@1..4 "foo"
                Token(Whitespace)@4..5 " "
                Context(Args)@5..10
                  Token(IdLower)@5..8 "bar"
                  Token(Whitespace)@8..9 " "
                  Token(Number)@9..10 "3""##]],
    )
}

#[test]
fn parse_type_match() {
    check(
        "@Number num",
        expect![[r#"
            Context(Pattern)@0..11
              Context(TypeMatch)@0..11
                Token(SymAt)@0..1 "@"
                Token(IdUpper)@1..7 "Number"
                Token(Whitespace)@7..8 " "
                Context(Pattern)@8..11
                  Token(IdLower)@8..11 "num""#]],
    )
}

mod literal {
    use super::*;

    #[test]
    fn parse_number() {
        check(
            "143",
            expect![[r#"
            Context(Pattern)@0..3
              Token(Number)@0..3 "143""#]],
        )
    }

    #[test]
    fn parse_simple_string() {
        check(
            r#""Hello World""#,
            expect![[r#"
            Context(Pattern)@0..13
              Context(String)@0..13
                Token(SymDoubleQuote)@0..1 "\""
                StringToken(Text)@1..12 "Hello World"
                StringToken(Delimiter)@12..13 "\"""#]],
        )
    }

    #[test]
    fn parse_string_with_escapes() {
        check(
            r#""Hello\n\tWorld""#,
            expect![[r#"
            Context(Pattern)@0..16
              Context(String)@0..16
                Token(SymDoubleQuote)@0..1 "\""
                StringToken(Text)@1..6 "Hello"
                StringToken(Escape)@6..8 "\\n"
                StringToken(Escape)@8..10 "\\t"
                StringToken(Text)@10..15 "World"
                StringToken(Delimiter)@15..16 "\"""#]],
        )
    }

    #[test]
    fn parse_nested_string() {
        check(
            r#""Hello\n${"\tworld \${text}"}""#,
            expect![[r#"
            Context(Pattern)@0..30
              Context(String)@0..30
                Token(SymDoubleQuote)@0..1 "\""
                StringToken(Text)@1..6 "Hello"
                StringToken(Escape)@6..8 "\\n"
                StringToken(ExprStart)@8..10 "${"
                Context(Expr)@10..28
                  Context(String)@10..28
                    Token(SymDoubleQuote)@10..11 "\""
                    StringToken(Escape)@11..13 "\\t"
                    StringToken(Text)@13..19 "world "
                    StringToken(Escape)@19..21 "\\$"
                    StringToken(Text)@21..27 "{text}"
                    StringToken(Delimiter)@27..28 "\""
                Token(SymRBrace)@28..29 "}"
                StringToken(Delimiter)@29..30 "\"""#]],
        )
    }

    #[test]
    fn parse_array() {
        check(
            "[foo, bar]",
            expect![[r#"
                Context(Pattern)@0..10
                  Context(Array)@0..10
                    Token(SymLBracket)@0..1 "["
                    Context(Item)@1..4
                      Token(IdLower)@1..4 "foo"
                    Token(SymComma)@4..5 ","
                    Token(Whitespace)@5..6 " "
                    Context(Item)@6..9
                      Token(IdLower)@6..9 "bar"
                    Token(SymRBracket)@9..10 "]""#]],
        )
    }
    #[test]
    fn parse_record() {
        check(
            "{foo, bar: baz}",
            expect![[r#"
                Context(Pattern)@0..15
                  Context(Record)@0..15
                    Token(SymLBrace)@0..1 "{"
                    Context(Field)@1..4
                      Token(IdLower)@1..4 "foo"
                    Token(SymComma)@4..5 ","
                    Context(Field)@5..14
                      Token(Whitespace)@5..6 " "
                      Token(IdLower)@6..9 "bar"
                      Token(SymColon)@9..10 ":"
                      Token(Whitespace)@10..11 " "
                      Token(IdLower)@11..14 "baz"
                    Token(SymRBrace)@14..15 "}""#]],
        )
    }
}
