use crate::parse_expression;
use expect_test::expect;

fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse_expression(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[test]
fn parse_placeholder() {
    check(
        "_",
        expect![[r#"
        Context(Expr)@0..1
          Token(Placeholder)@0..1 "_""#]],
    )
}

mod literal {
    use super::*;

    #[test]
    fn parse_number() {
        check(
            "143",
            expect![[r#"
            Context(Expr)@0..3
              Token(Number)@0..3 "143""#]],
        )
    }

    #[test]
    fn parse_simple_string() {
        check(
            r#""Hello World""#,
            expect![[r#"
            Context(Expr)@0..13
              Context(String)@0..13
                Token(DoubleQuote)@0..1 "\""
                StringToken(Text)@1..12 "Hello World"
                StringToken(Delimiter)@12..13 "\"""#]],
        )
    }

    #[test]
    fn parse_string_with_escapes() {
        check(
            r#""Hello\n\tWorld""#,
            expect![[r#"
            Context(Expr)@0..16
              Context(String)@0..16
                Token(DoubleQuote)@0..1 "\""
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
            Context(Expr)@0..30
              Context(String)@0..30
                Token(DoubleQuote)@0..1 "\""
                StringToken(Text)@1..6 "Hello"
                StringToken(Escape)@6..8 "\\n"
                StringToken(ExprStart)@8..10 "${"
                Context(Expr)@10..28
                  Context(String)@10..28
                    Token(DoubleQuote)@10..11 "\""
                    StringToken(Escape)@11..13 "\\t"
                    StringToken(Text)@13..19 "world "
                    StringToken(Escape)@19..21 "\\$"
                    StringToken(Text)@21..27 "{text}"
                    StringToken(Delimiter)@27..28 "\""
                Token(CurlyClose)@28..29 "}"
                StringToken(Delimiter)@29..30 "\"""#]],
        )
    }

    #[test]
    fn parse_array() {
        check(
            "[foo, bar]",
            expect![[r#"
            Context(Expr)@0..9
              Context(Array)@0..9
                Token(SquareOpen)@0..1 "["
                Token(VarName)@1..4 "foo"
                Token(Comma)@4..5 ","
                Token(VarName)@5..8 "bar"
                Token(SquareClose)@8..9 "]""#]],
        )
    }
    #[test]
    fn parse_record() {
        check(
            "{foo, bar: baz}",
            expect![[r#"
            Context(Expr)@0..13
              Context(Record)@0..13
                Token(CurlyOpen)@0..1 "{"
                Context(Field)@1..5
                  Token(VarName)@1..4 "foo"
                  Token(Comma)@4..5 ","
                Context(Field)@5..12
                  Token(VarName)@5..8 "bar"
                  Token(Colon)@8..9 ":"
                  Token(VarName)@9..12 "baz"
                Token(CurlyClose)@12..13 "}""#]],
        )
    }
}

mod operator {
    use super::*;

    #[test]
    fn parse_simple_binop() {
        check(
            "2+3",
            expect![[r#"
            Context(Expr)@0..3
              Context(BinOp)@0..3
                Token(Number)@0..1 "2"
                Token(OpAdd)@1..2 "+"
                Token(Number)@2..3 "3""#]],
        )
    }

    #[test]
    fn parse_mixed_binop() {
        check(
            "2+3*4-5",
            expect![[r#"
            Context(Expr)@0..7
              Context(BinOp)@0..7
                Context(BinOp)@0..5
                  Token(Number)@0..1 "2"
                  Token(OpAdd)@1..2 "+"
                  Context(BinOp)@2..5
                    Token(Number)@2..3 "3"
                    Token(OpMul)@3..4 "*"
                    Token(Number)@4..5 "4"
                Token(OpSub)@5..6 "-"
                Token(Number)@6..7 "5""#]],
        )
    }

    #[test]
    fn parse_negate_num() {
        check(
            "-1",
            expect![[r#"
            Context(Expr)@0..2
              Context(PrefixOp)@0..2
                Token(OpSub)@0..1 "-"
                Token(Number)@1..2 "1""#]],
        )
    }

    #[test]
    fn parse_negate_var() {
        check(
            "-foo",
            expect![[r#"
            Context(Expr)@0..4
              Context(PrefixOp)@0..4
                Token(OpSub)@0..1 "-"
                Token(VarName)@1..4 "foo""#]],
        )
    }

    #[test]
    fn parse_paren_precedence() {
        check(
            "1 + 2 * 3 / (5 - 2)",
            expect![[r#"
            Context(Expr)@0..11
              Context(BinOp)@0..11
                Token(Number)@0..1 "1"
                Token(OpAdd)@1..2 "+"
                Context(BinOp)@2..11
                  Context(BinOp)@2..5
                    Token(Number)@2..3 "2"
                    Token(OpMul)@3..4 "*"
                    Token(Number)@4..5 "3"
                  Token(OpDiv)@5..6 "/"
                  Context(Expr)@6..11
                    Token(ParenOpen)@6..7 "("
                    Context(BinOp)@7..10
                      Token(Number)@7..8 "5"
                      Token(OpSub)@8..9 "-"
                      Token(Number)@9..10 "2"
                    Token(ParenClose)@10..11 ")""#]],
        )
    }

    #[test]
    fn parse_commented_expr() {
        check(
            "1 + 2 // Not applied as a single term
            * 3 // = 6
            /
            (5 - -2) // 5 - (-2) = 6",
            expect![[r#"
                Context(Expr)@0..64
                  Context(BinOp)@0..64
                    Token(Number)@0..1 "1"
                    Token(OpAdd)@1..2 "+"
                    Context(BinOp)@2..64
                      Context(BinOp)@2..42
                        Token(Number)@2..3 "2"
                        Token(Comment)@3..34 "// Not applied as a s ..."
                        Token(OpMul)@34..35 "*"
                        Token(Number)@35..36 "3"
                        Token(Comment)@36..42 "// = 6"
                      Token(OpDiv)@42..43 "/"
                      Context(Expr)@43..49
                        Token(ParenOpen)@43..44 "("
                        Context(BinOp)@44..48
                          Token(Number)@44..45 "5"
                          Token(OpSub)@45..46 "-"
                          Context(PrefixOp)@46..48
                            Token(OpSub)@46..47 "-"
                            Token(Number)@47..48 "2"
                        Token(ParenClose)@48..49 ")"
                      Token(Comment)@49..64 "// 5 - (-2) = 6""#]],
        )
    }
}

mod variable {
    use super::*;

    #[test]
    fn parse_local() {
        check(
            "varName1",
            expect![[r#"
            Context(Expr)@0..8
              Token(VarName)@0..8 "varName1""#]],
        )
    }

    #[test]
    fn parse_scoped() {
        check(
            "Name.Space.foo",
            expect![[r#"
            Context(Expr)@0..14
              Context(Scoped)@0..14
                Token(Namespace)@0..4 "Name"
                Token(Period)@4..5 "."
                Token(Namespace)@5..10 "Space"
                Token(Period)@10..11 "."
                Token(VarName)@11..14 "foo""#]],
        )
    }
}

#[test]
fn parse_let() {
    check(
        "let foo = 1; foo + 3",
        expect![[r#"
        Context(Expr)@0..14
          Context(Declaration)@0..14
            Token(KWLet)@0..3 "let"
            Token(VarName)@3..6 "foo"
            Token(OpAssign)@6..7 "="
            Context(Expr)@7..8
              Token(Number)@7..8 "1"
            Token(SemiColon)@8..9 ";"
            Context(BinOp)@9..14
              Token(VarName)@9..12 "foo"
              Token(OpAdd)@12..13 "+"
              Token(Number)@13..14 "3""#]],
    )
}

#[test]
fn parse_access() {
    check(
        "foo.bar.baz",
        expect![[r#"
        Context(Expr)@0..11
          Context(Access)@0..11
            Context(Access)@0..7
              Token(VarName)@0..3 "foo"
              Token(Period)@3..4 "."
              Token(VarName)@4..7 "bar"
            Token(Period)@7..8 "."
            Token(VarName)@8..11 "baz""#]],
    )
}

#[test]
fn parse_application() {
    check(
        "foo bar (3 -1)",
        expect![[r#"
        Context(Expr)@0..13
          Context(Application)@0..13
            Context(Application)@0..7
              Token(VarName)@0..3 "foo"
              Token(Whitespace)@3..4 " "
              Token(VarName)@4..7 "bar"
            Token(Whitespace)@7..8 " "
            Context(Expr)@8..13
              Token(ParenOpen)@8..9 "("
              Context(BinOp)@9..12
                Token(Number)@9..10 "3"
                Token(OpSub)@10..11 "-"
                Token(Number)@11..12 "1"
              Token(ParenClose)@12..13 ")""#]],
    )
}
