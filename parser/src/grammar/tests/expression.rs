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
                Context(Expr)@0..10
                  Context(Array)@0..10
                    Token(SquareOpen)@0..1 "["
                    Token(VarName)@1..4 "foo"
                    Token(Comma)@4..5 ","
                    Token(Whitespace)@5..6 " "
                    Token(VarName)@6..9 "bar"
                    Token(SquareClose)@9..10 "]""#]],
        )
    }
    #[test]
    fn parse_record() {
        check(
            "{foo, bar: baz}",
            expect![[r#"
                Context(Expr)@0..15
                  Context(Record)@0..15
                    Token(CurlyOpen)@0..1 "{"
                    Context(Field)@1..5
                      Token(VarName)@1..4 "foo"
                      Token(Comma)@4..5 ","
                    Context(Field)@5..14
                      Token(Whitespace)@5..6 " "
                      Token(VarName)@6..9 "bar"
                      Token(Colon)@9..10 ":"
                      Token(Whitespace)@10..11 " "
                      Token(VarName)@11..14 "baz"
                    Token(CurlyClose)@14..15 "}""#]],
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
                Context(Expr)@0..19
                  Context(BinOp)@0..19
                    Token(Number)@0..1 "1"
                    Token(Whitespace)@1..2 " "
                    Token(OpAdd)@2..3 "+"
                    Context(BinOp)@3..19
                      Context(BinOp)@3..10
                        Token(Whitespace)@3..4 " "
                        Token(Number)@4..5 "2"
                        Token(Whitespace)@5..6 " "
                        Token(OpMul)@6..7 "*"
                        Token(Whitespace)@7..8 " "
                        Token(Number)@8..9 "3"
                        Token(Whitespace)@9..10 " "
                      Token(OpDiv)@10..11 "/"
                      Token(Whitespace)@11..12 " "
                      Context(Expr)@12..19
                        Token(ParenOpen)@12..13 "("
                        Context(BinOp)@13..18
                          Token(Number)@13..14 "5"
                          Token(Whitespace)@14..15 " "
                          Token(OpSub)@15..16 "-"
                          Token(Whitespace)@16..17 " "
                          Token(Number)@17..18 "2"
                        Token(ParenClose)@18..19 ")""#]],
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
                Context(Expr)@0..111
                  Context(BinOp)@0..111
                    Token(Number)@0..1 "1"
                    Token(Whitespace)@1..2 " "
                    Token(OpAdd)@2..3 "+"
                    Context(BinOp)@3..111
                      Context(BinOp)@3..73
                        Token(Whitespace)@3..4 " "
                        Token(Number)@4..5 "2"
                        Token(Whitespace)@5..6 " "
                        Token(Comment)@6..37 "// Not applied as a s ..."
                        Token(Whitespace)@37..50 "\n            "
                        Token(OpMul)@50..51 "*"
                        Token(Whitespace)@51..52 " "
                        Token(Number)@52..53 "3"
                        Token(Whitespace)@53..54 " "
                        Token(Comment)@54..60 "// = 6"
                        Token(Whitespace)@60..73 "\n            "
                      Token(OpDiv)@73..74 "/"
                      Token(Whitespace)@74..87 "\n            "
                      Context(Expr)@87..95
                        Token(ParenOpen)@87..88 "("
                        Context(BinOp)@88..94
                          Token(Number)@88..89 "5"
                          Token(Whitespace)@89..90 " "
                          Token(OpSub)@90..91 "-"
                          Token(Whitespace)@91..92 " "
                          Context(PrefixOp)@92..94
                            Token(OpSub)@92..93 "-"
                            Token(Number)@93..94 "2"
                        Token(ParenClose)@94..95 ")"
                      Token(Whitespace)@95..96 " "
                      Token(Comment)@96..111 "// 5 - (-2) = 6""#]],
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
            Context(Expr)@0..20
              Context(Declaration)@0..20
                Token(KWLet)@0..3 "let"
                Token(Whitespace)@3..4 " "
                Token(VarName)@4..7 "foo"
                Token(Whitespace)@7..8 " "
                Token(OpAssign)@8..9 "="
                Context(Expr)@9..11
                  Token(Whitespace)@9..10 " "
                  Token(Number)@10..11 "1"
                Token(SemiColon)@11..12 ";"
                Context(BinOp)@12..20
                  Token(Whitespace)@12..13 " "
                  Token(VarName)@13..16 "foo"
                  Token(Whitespace)@16..17 " "
                  Token(OpAdd)@17..18 "+"
                  Token(Whitespace)@18..19 " "
                  Token(Number)@19..20 "3""#]],
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
            Context(Expr)@0..14
              Context(Application)@0..14
                Context(Application)@0..7
                  Token(VarName)@0..3 "foo"
                  Token(Whitespace)@3..4 " "
                  Token(VarName)@4..7 "bar"
                Token(Whitespace)@7..8 " "
                Context(Expr)@8..14
                  Token(ParenOpen)@8..9 "("
                  Context(BinOp)@9..13
                    Token(Number)@9..10 "3"
                    Token(Whitespace)@10..11 " "
                    Token(OpSub)@11..12 "-"
                    Token(Number)@12..13 "1"
                  Token(ParenClose)@13..14 ")""#]],
    )
}

#[test]
fn parse_constructor() {
    check(
        "#foo bar (3 -1)",
        expect![[r##"
            Context(Expr)@0..15
              Context(Constructor)@0..15
                Token(Hash)@0..1 "#"
                Token(VarName)@1..4 "foo"
                Token(Whitespace)@4..5 " "
                Context(Args)@5..15
                  Token(VarName)@5..8 "bar"
                  Token(Whitespace)@8..9 " "
                  Context(Expr)@9..15
                    Token(ParenOpen)@9..10 "("
                    Context(BinOp)@10..14
                      Token(Number)@10..11 "3"
                      Token(Whitespace)@11..12 " "
                      Token(OpSub)@12..13 "-"
                      Token(Number)@13..14 "1"
                    Token(ParenClose)@14..15 ")""##]],
    )
}

#[test]
fn parse_conditional() {
    check(
        "if a and b then c + 4 else 2 * c",
        expect![[r#"
            Context(Expr)@0..32
              Context(Conditional)@0..32
                Token(KWIf)@0..2 "if"
                Context(Condition)@2..11
                  Context(BinOp)@2..11
                    Token(Whitespace)@2..3 " "
                    Token(VarName)@3..4 "a"
                    Token(Whitespace)@4..5 " "
                    Token(OpAnd)@5..8 "and"
                    Token(Whitespace)@8..9 " "
                    Token(VarName)@9..10 "b"
                    Token(Whitespace)@10..11 " "
                Token(KWThen)@11..15 "then"
                Context(Then)@15..22
                  Context(BinOp)@15..22
                    Token(Whitespace)@15..16 " "
                    Token(VarName)@16..17 "c"
                    Token(Whitespace)@17..18 " "
                    Token(OpAdd)@18..19 "+"
                    Token(Whitespace)@19..20 " "
                    Token(Number)@20..21 "4"
                    Token(Whitespace)@21..22 " "
                Token(KWElse)@22..26 "else"
                Context(Else)@26..32
                  Context(BinOp)@26..32
                    Token(Whitespace)@26..27 " "
                    Token(Number)@27..28 "2"
                    Token(Whitespace)@28..29 " "
                    Token(OpMul)@29..30 "*"
                    Token(Whitespace)@30..31 " "
                    Token(VarName)@31..32 "c""#]],
    )
}

#[test]
fn parse_nested_conditional() {
    check(
        "if if a and b then c + 4 else 2 * c then if d or e then 2 else 3 else f",
        expect![[r#"
            Context(Expr)@0..71
              Context(Conditional)@0..71
                Token(KWIf)@0..2 "if"
                Context(Condition)@2..36
                  Token(Whitespace)@2..3 " "
                  Context(Conditional)@3..36
                    Token(KWIf)@3..5 "if"
                    Context(Condition)@5..14
                      Context(BinOp)@5..14
                        Token(Whitespace)@5..6 " "
                        Token(VarName)@6..7 "a"
                        Token(Whitespace)@7..8 " "
                        Token(OpAnd)@8..11 "and"
                        Token(Whitespace)@11..12 " "
                        Token(VarName)@12..13 "b"
                        Token(Whitespace)@13..14 " "
                    Token(KWThen)@14..18 "then"
                    Context(Then)@18..25
                      Context(BinOp)@18..25
                        Token(Whitespace)@18..19 " "
                        Token(VarName)@19..20 "c"
                        Token(Whitespace)@20..21 " "
                        Token(OpAdd)@21..22 "+"
                        Token(Whitespace)@22..23 " "
                        Token(Number)@23..24 "4"
                        Token(Whitespace)@24..25 " "
                    Token(KWElse)@25..29 "else"
                    Context(Else)@29..36
                      Context(BinOp)@29..36
                        Token(Whitespace)@29..30 " "
                        Token(Number)@30..31 "2"
                        Token(Whitespace)@31..32 " "
                        Token(OpMul)@32..33 "*"
                        Token(Whitespace)@33..34 " "
                        Token(VarName)@34..35 "c"
                        Token(Whitespace)@35..36 " "
                Token(KWThen)@36..40 "then"
                Context(Then)@40..65
                  Token(Whitespace)@40..41 " "
                  Context(Conditional)@41..65
                    Token(KWIf)@41..43 "if"
                    Context(Condition)@43..51
                      Context(BinOp)@43..51
                        Token(Whitespace)@43..44 " "
                        Token(VarName)@44..45 "d"
                        Token(Whitespace)@45..46 " "
                        Token(OpOr)@46..48 "or"
                        Token(Whitespace)@48..49 " "
                        Token(VarName)@49..50 "e"
                        Token(Whitespace)@50..51 " "
                    Token(KWThen)@51..55 "then"
                    Context(Then)@55..58
                      Token(Whitespace)@55..56 " "
                      Token(Number)@56..57 "2"
                      Token(Whitespace)@57..58 " "
                    Token(KWElse)@58..62 "else"
                    Context(Else)@62..65
                      Token(Whitespace)@62..63 " "
                      Token(Number)@63..64 "3"
                      Token(Whitespace)@64..65 " "
                Token(KWElse)@65..69 "else"
                Context(Else)@69..71
                  Token(Whitespace)@69..70 " "
                  Token(VarName)@70..71 "f""#]],
    )
}

#[test]
fn parse_where() {
    check(
        r#"where foo is 1 => "hello" is 2 => "world" _ => "!""#,
        expect![[r#"
        Context(Expr)@0..14
          Context(Constructor)@0..14
            Token(Hash)@0..1 ""
            Token(VarName)@1..4 "foo"
            Context(Args)@4..14
              Token(Whitespace)@4..5 " "
              Token(VarName)@5..8 "bar"
              Token(Whitespace)@8..9 " "
              Context(Expr)@9..14
                Token(ParenOpen)@9..10 "("
                Context(BinOp)@10..13
                  Token(Number)@10..11 "3"
                  Token(OpSub)@11..12 "-"
                  Token(Number)@12..13 "1"
                Token(ParenClose)@13..14 ")""#]],
    )
}

#[test]
fn parse_lambda() {
    check(
        "fun a b => a * (3 - b)",
        expect![[r#"
        Context(Expr)@0..14
          Context(Lambda)@0..14
            Token(Hash)@0..1 ""
            Token(VarName)@1..4 "foo"
            Context(Args)@4..14
              Token(Whitespace)@4..5 " "
              Token(VarName)@5..8 "bar"
              Token(Whitespace)@8..9 " "
              Context(Expr)@9..14
                Token(ParenOpen)@9..10 "("
                Context(BinOp)@10..13
                  Token(Number)@10..11 "3"
                  Token(OpSub)@11..12 "-"
                  Token(Number)@12..13 "1"
                Token(ParenClose)@13..14 ")""#]],
    )
}
