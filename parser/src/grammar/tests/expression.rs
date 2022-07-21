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
                    Context(Item)@1..4
                      Token(VarName)@1..4 "foo"
                    Token(Comma)@4..5 ","
                    Token(Whitespace)@5..6 " "
                    Context(Item)@6..9
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
                        Token(Comment)@6..38 "// Not applied as a s ..."
                        Token(Whitespace)@38..50 "            "
                        Token(OpMul)@50..51 "*"
                        Token(Whitespace)@51..52 " "
                        Token(Number)@52..53 "3"
                        Token(Whitespace)@53..54 " "
                        Token(Comment)@54..61 "// = 6\n"
                        Token(Whitespace)@61..73 "            "
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
        r#"let [foo, _] = [1, "bar"]; foo + 3"#,
        expect![[r#"
            Context(Expr)@0..34
              Context(Declaration)@0..34
                Token(KWLet)@0..3 "let"
                Context(Pattern)@3..12
                  Token(Whitespace)@3..4 " "
                  Context(Array)@4..12
                    Token(SquareOpen)@4..5 "["
                    Context(Item)@5..8
                      Token(VarName)@5..8 "foo"
                    Token(Comma)@8..9 ","
                    Token(Whitespace)@9..10 " "
                    Context(Item)@10..11
                      Token(Placeholder)@10..11 "_"
                    Token(SquareClose)@11..12 "]"
                Token(Whitespace)@12..13 " "
                Token(OpAssign)@13..14 "="
                Context(Expr)@14..25
                  Token(Whitespace)@14..15 " "
                  Context(Array)@15..25
                    Token(SquareOpen)@15..16 "["
                    Context(Item)@16..17
                      Token(Number)@16..17 "1"
                    Token(Comma)@17..18 ","
                    Token(Whitespace)@18..19 " "
                    Context(Item)@19..24
                      Context(String)@19..24
                        Token(DoubleQuote)@19..20 "\""
                        StringToken(Text)@20..23 "bar"
                        StringToken(Delimiter)@23..24 "\""
                    Token(SquareClose)@24..25 "]"
                Token(SemiColon)@25..26 ";"
                Context(BinOp)@26..34
                  Token(Whitespace)@26..27 " "
                  Token(VarName)@27..30 "foo"
                  Token(Whitespace)@30..31 " "
                  Token(OpAdd)@31..32 "+"
                  Token(Whitespace)@32..33 " "
                  Token(Number)@33..34 "3""#]],
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
        r#"where foo is 1 => "hello" is 2 if bar > foo => "world" is #just baz => "baz" is _ => "!""#,
        expect![[r##"
            Context(Expr)@0..88
              Context(Where)@0..88
                Token(KWWhere)@0..5 "where"
                Context(Expr)@5..10
                  Token(Whitespace)@5..6 " "
                  Token(VarName)@6..9 "foo"
                  Token(Whitespace)@9..10 " "
                Context(Branch)@10..26
                  Token(KWIs)@10..12 "is"
                  Context(Pattern)@12..14
                    Token(Whitespace)@12..13 " "
                    Token(Number)@13..14 "1"
                  Token(Whitespace)@14..15 " "
                  Token(OpArrow)@15..17 "=>"
                  Context(Expr)@17..26
                    Token(Whitespace)@17..18 " "
                    Context(String)@18..25
                      Token(DoubleQuote)@18..19 "\""
                      StringToken(Text)@19..24 "hello"
                      StringToken(Delimiter)@24..25 "\""
                    Token(Whitespace)@25..26 " "
                Context(Branch)@26..55
                  Token(KWIs)@26..28 "is"
                  Context(Pattern)@28..30
                    Token(Whitespace)@28..29 " "
                    Token(Number)@29..30 "2"
                  Token(Whitespace)@30..31 " "
                  Context(Guard)@31..44
                    Token(KWIf)@31..33 "if"
                    Context(BinOp)@33..44
                      Token(Whitespace)@33..34 " "
                      Token(VarName)@34..37 "bar"
                      Token(Whitespace)@37..38 " "
                      Token(OpGt)@38..39 ">"
                      Token(Whitespace)@39..40 " "
                      Token(VarName)@40..43 "foo"
                      Token(Whitespace)@43..44 " "
                  Token(OpArrow)@44..46 "=>"
                  Context(Expr)@46..55
                    Token(Whitespace)@46..47 " "
                    Context(String)@47..54
                      Token(DoubleQuote)@47..48 "\""
                      StringToken(Text)@48..53 "world"
                      StringToken(Delimiter)@53..54 "\""
                    Token(Whitespace)@54..55 " "
                Context(Branch)@55..77
                  Token(KWIs)@55..57 "is"
                  Context(Pattern)@57..68
                    Token(Whitespace)@57..58 " "
                    Context(Constructor)@58..68
                      Token(Hash)@58..59 "#"
                      Token(VarName)@59..63 "just"
                      Context(Args)@63..68
                        Token(Whitespace)@63..64 " "
                        Token(VarName)@64..67 "baz"
                        Token(Whitespace)@67..68 " "
                  Token(OpArrow)@68..70 "=>"
                  Context(Expr)@70..77
                    Token(Whitespace)@70..71 " "
                    Context(String)@71..76
                      Token(DoubleQuote)@71..72 "\""
                      StringToken(Text)@72..75 "baz"
                      StringToken(Delimiter)@75..76 "\""
                    Token(Whitespace)@76..77 " "
                Context(Branch)@77..88
                  Token(KWIs)@77..79 "is"
                  Context(Pattern)@79..81
                    Token(Whitespace)@79..80 " "
                    Token(Placeholder)@80..81 "_"
                  Token(Whitespace)@81..82 " "
                  Token(OpArrow)@82..84 "=>"
                  Context(Expr)@84..88
                    Token(Whitespace)@84..85 " "
                    Context(String)@85..88
                      Token(DoubleQuote)@85..86 "\""
                      StringToken(Text)@86..87 "!"
                      StringToken(Delimiter)@87..88 "\"""##]],
    )
}

#[test]
fn parse_lambda() {
    check(
        "fun a b => a * (3 - b)",
        expect![[r#"
            Context(Expr)@0..22
              Context(Lambda)@0..22
                Token(KWFun)@0..3 "fun"
                Context(Params)@3..10
                  Context(Pattern)@3..5
                    Token(Whitespace)@3..4 " "
                    Token(VarName)@4..5 "a"
                  Token(Whitespace)@5..6 " "
                  Context(Pattern)@6..7
                    Token(VarName)@6..7 "b"
                  Token(Whitespace)@7..8 " "
                  Token(OpArrow)@8..10 "=>"
                Context(BinOp)@10..22
                  Token(Whitespace)@10..11 " "
                  Token(VarName)@11..12 "a"
                  Token(Whitespace)@12..13 " "
                  Token(OpMul)@13..14 "*"
                  Token(Whitespace)@14..15 " "
                  Context(Expr)@15..22
                    Token(ParenOpen)@15..16 "("
                    Context(BinOp)@16..21
                      Token(Number)@16..17 "3"
                      Token(Whitespace)@17..18 " "
                      Token(OpSub)@18..19 "-"
                      Token(Whitespace)@19..20 " "
                      Token(VarName)@20..21 "b"
                    Token(ParenClose)@21..22 ")""#]],
    )
}
