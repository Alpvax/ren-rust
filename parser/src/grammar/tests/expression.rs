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
              Token(SymUnderscore)@0..1 "_""#]],
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
            Context(Expr)@0..16
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
            Context(Expr)@0..30
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
                Context(Expr)@0..10
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
                Context(Expr)@0..15
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
                Token(IdLower)@1..4 "foo""#]],
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
                      Context(Parenthesised)@12..19
                        Token(SymLParen)@12..13 "("
                        Context(BinOp)@13..18
                          Token(Number)@13..14 "5"
                          Token(Whitespace)@14..15 " "
                          Token(OpSub)@15..16 "-"
                          Token(Whitespace)@16..17 " "
                          Token(Number)@17..18 "2"
                        Token(SymRParen)@18..19 ")""#]],
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
                      Context(Parenthesised)@87..95
                        Token(SymLParen)@87..88 "("
                        Context(BinOp)@88..94
                          Token(Number)@88..89 "5"
                          Token(Whitespace)@89..90 " "
                          Token(OpSub)@90..91 "-"
                          Token(Whitespace)@91..92 " "
                          Context(PrefixOp)@92..94
                            Token(OpSub)@92..93 "-"
                            Token(Number)@93..94 "2"
                        Token(SymRParen)@94..95 ")"
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
                  Token(IdLower)@0..8 "varName1""#]],
        )
    }

    #[test]
    fn parse_scoped() {
        check(
            "Name.Space.foo",
            expect![[r#"
                Context(Expr)@0..14
                  Context(Scoped)@0..14
                    Token(IdUpper)@0..4 "Name"
                    Token(SymDot)@4..5 "."
                    Token(IdUpper)@5..10 "Space"
                    Token(SymDot)@10..11 "."
                    Token(IdLower)@11..14 "foo""#]],
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
                    Token(SymLBracket)@4..5 "["
                    Context(Item)@5..8
                      Token(IdLower)@5..8 "foo"
                    Token(SymComma)@8..9 ","
                    Token(Whitespace)@9..10 " "
                    Context(Item)@10..11
                      Token(SymUnderscore)@10..11 "_"
                    Token(SymRBracket)@11..12 "]"
                Token(Whitespace)@12..13 " "
                Token(SymEquals)@13..14 "="
                Context(Expr)@14..25
                  Token(Whitespace)@14..15 " "
                  Context(Array)@15..25
                    Token(SymLBracket)@15..16 "["
                    Context(Item)@16..17
                      Token(Number)@16..17 "1"
                    Token(SymComma)@17..18 ","
                    Token(Whitespace)@18..19 " "
                    Context(Item)@19..24
                      Context(String)@19..24
                        Token(SymDoubleQuote)@19..20 "\""
                        StringToken(Text)@20..23 "bar"
                        StringToken(Delimiter)@23..24 "\""
                    Token(SymRBracket)@24..25 "]"
                Token(OpSeq)@25..26 ";"
                Context(BinOp)@26..34
                  Token(Whitespace)@26..27 " "
                  Token(IdLower)@27..30 "foo"
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
                  Token(IdLower)@0..3 "foo"
                  Token(SymDot)@3..4 "."
                  Token(IdLower)@4..7 "bar"
                Token(SymDot)@7..8 "."
                Token(IdLower)@8..11 "baz""#]],
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
                  Token(IdLower)@0..3 "foo"
                  Token(Whitespace)@3..4 " "
                  Token(IdLower)@4..7 "bar"
                Token(Whitespace)@7..8 " "
                Context(Parenthesised)@8..14
                  Token(SymLParen)@8..9 "("
                  Context(BinOp)@9..13
                    Token(Number)@9..10 "3"
                    Token(Whitespace)@10..11 " "
                    Token(OpSub)@11..12 "-"
                    Token(Number)@12..13 "1"
                  Token(SymRParen)@13..14 ")""#]],
    )
}

#[test]
fn parse_constructor() {
    check(
        "#foo bar (3 -1)",
        expect![[r##"
            Context(Expr)@0..15
              Context(Constructor)@0..15
                Token(SymHash)@0..1 "#"
                Token(IdLower)@1..4 "foo"
                Token(Whitespace)@4..5 " "
                Context(Args)@5..15
                  Token(IdLower)@5..8 "bar"
                  Token(Whitespace)@8..9 " "
                  Context(Parenthesised)@9..15
                    Token(SymLParen)@9..10 "("
                    Context(BinOp)@10..14
                      Token(Number)@10..11 "3"
                      Token(Whitespace)@11..12 " "
                      Token(OpSub)@12..13 "-"
                      Token(Number)@13..14 "1"
                    Token(SymRParen)@14..15 ")""##]],
    )
}

#[test]
fn parse_conditional() {
    check(
        "if a & b then c + 4 else 2 * c",
        expect![[r#"
            Context(Expr)@0..30
              Context(Conditional)@0..30
                Token(KWIf)@0..2 "if"
                Context(Condition)@2..9
                  Context(BinOp)@2..9
                    Token(Whitespace)@2..3 " "
                    Token(IdLower)@3..4 "a"
                    Token(Whitespace)@4..5 " "
                    Token(OpAnd)@5..6 "&"
                    Token(Whitespace)@6..7 " "
                    Token(IdLower)@7..8 "b"
                    Token(Whitespace)@8..9 " "
                Token(KWThen)@9..13 "then"
                Context(Then)@13..20
                  Context(BinOp)@13..20
                    Token(Whitespace)@13..14 " "
                    Token(IdLower)@14..15 "c"
                    Token(Whitespace)@15..16 " "
                    Token(OpAdd)@16..17 "+"
                    Token(Whitespace)@17..18 " "
                    Token(Number)@18..19 "4"
                    Token(Whitespace)@19..20 " "
                Token(KWElse)@20..24 "else"
                Context(Else)@24..30
                  Context(BinOp)@24..30
                    Token(Whitespace)@24..25 " "
                    Token(Number)@25..26 "2"
                    Token(Whitespace)@26..27 " "
                    Token(OpMul)@27..28 "*"
                    Token(Whitespace)@28..29 " "
                    Token(IdLower)@29..30 "c""#]],
    )
}

#[test]
fn parse_nested_conditional() {
    check(
        "if if a & b then c + 4 else 2 * c then if d | e then 2 else 3 else f",
        expect![[r#"
            Context(Expr)@0..68
              Context(Conditional)@0..68
                Token(KWIf)@0..2 "if"
                Context(Condition)@2..34
                  Token(Whitespace)@2..3 " "
                  Context(Conditional)@3..34
                    Token(KWIf)@3..5 "if"
                    Context(Condition)@5..12
                      Context(BinOp)@5..12
                        Token(Whitespace)@5..6 " "
                        Token(IdLower)@6..7 "a"
                        Token(Whitespace)@7..8 " "
                        Token(OpAnd)@8..9 "&"
                        Token(Whitespace)@9..10 " "
                        Token(IdLower)@10..11 "b"
                        Token(Whitespace)@11..12 " "
                    Token(KWThen)@12..16 "then"
                    Context(Then)@16..23
                      Context(BinOp)@16..23
                        Token(Whitespace)@16..17 " "
                        Token(IdLower)@17..18 "c"
                        Token(Whitespace)@18..19 " "
                        Token(OpAdd)@19..20 "+"
                        Token(Whitespace)@20..21 " "
                        Token(Number)@21..22 "4"
                        Token(Whitespace)@22..23 " "
                    Token(KWElse)@23..27 "else"
                    Context(Else)@27..34
                      Context(BinOp)@27..34
                        Token(Whitespace)@27..28 " "
                        Token(Number)@28..29 "2"
                        Token(Whitespace)@29..30 " "
                        Token(OpMul)@30..31 "*"
                        Token(Whitespace)@31..32 " "
                        Token(IdLower)@32..33 "c"
                        Token(Whitespace)@33..34 " "
                Token(KWThen)@34..38 "then"
                Context(Then)@38..62
                  Token(Whitespace)@38..39 " "
                  Context(Conditional)@39..62
                    Token(KWIf)@39..41 "if"
                    Context(Condition)@41..48
                      Context(BinOp)@41..48
                        Token(Whitespace)@41..42 " "
                        Token(IdLower)@42..43 "d"
                        Token(Whitespace)@43..44 " "
                        Token(OpOr)@44..45 "|"
                        Token(Whitespace)@45..46 " "
                        Token(IdLower)@46..47 "e"
                        Token(Whitespace)@47..48 " "
                    Token(KWThen)@48..52 "then"
                    Context(Then)@52..55
                      Token(Whitespace)@52..53 " "
                      Token(Number)@53..54 "2"
                      Token(Whitespace)@54..55 " "
                    Token(KWElse)@55..59 "else"
                    Context(Else)@59..62
                      Token(Whitespace)@59..60 " "
                      Token(Number)@60..61 "3"
                      Token(Whitespace)@61..62 " "
                Token(KWElse)@62..66 "else"
                Context(Else)@66..68
                  Token(Whitespace)@66..67 " "
                  Token(IdLower)@67..68 "f""#]],
    )
}

#[test]
fn parse_switch() {
    check(
        r#"switch foo on case 1 -> "hello" case 2 if bar > foo -> "world" case #just baz -> "baz" case _ -> "!""#,
        expect![[r##"
            Context(Expr)@0..100
              Context(Switch)@0..100
                Token(KWSwitch)@0..6 "switch"
                Context(Expr)@6..11
                  Token(Whitespace)@6..7 " "
                  Token(IdLower)@7..10 "foo"
                  Token(Whitespace)@10..11 " "
                Token(KWOn)@11..13 "on"
                Context(Branch)@13..32
                  Token(Whitespace)@13..14 " "
                  Token(KWCase)@14..18 "case"
                  Context(Pattern)@18..20
                    Token(Whitespace)@18..19 " "
                    Token(Number)@19..20 "1"
                  Token(Whitespace)@20..21 " "
                  Token(SymArrow)@21..23 "->"
                  Context(Expr)@23..32
                    Token(Whitespace)@23..24 " "
                    Context(String)@24..31
                      Token(SymDoubleQuote)@24..25 "\""
                      StringToken(Text)@25..30 "hello"
                      StringToken(Delimiter)@30..31 "\""
                    Token(Whitespace)@31..32 " "
                Context(Branch)@32..63
                  Token(KWCase)@32..36 "case"
                  Context(Pattern)@36..38
                    Token(Whitespace)@36..37 " "
                    Token(Number)@37..38 "2"
                  Token(Whitespace)@38..39 " "
                  Context(Guard)@39..52
                    Token(KWIf)@39..41 "if"
                    Context(BinOp)@41..52
                      Token(Whitespace)@41..42 " "
                      Token(IdLower)@42..45 "bar"
                      Token(Whitespace)@45..46 " "
                      Token(OpGt)@46..47 ">"
                      Token(Whitespace)@47..48 " "
                      Token(IdLower)@48..51 "foo"
                      Token(Whitespace)@51..52 " "
                  Token(SymArrow)@52..54 "->"
                  Context(Expr)@54..63
                    Token(Whitespace)@54..55 " "
                    Context(String)@55..62
                      Token(SymDoubleQuote)@55..56 "\""
                      StringToken(Text)@56..61 "world"
                      StringToken(Delimiter)@61..62 "\""
                    Token(Whitespace)@62..63 " "
                Context(Branch)@63..87
                  Token(KWCase)@63..67 "case"
                  Context(Pattern)@67..78
                    Token(Whitespace)@67..68 " "
                    Context(Constructor)@68..78
                      Token(SymHash)@68..69 "#"
                      Token(IdLower)@69..73 "just"
                      Context(Args)@73..78
                        Token(Whitespace)@73..74 " "
                        Token(IdLower)@74..77 "baz"
                        Token(Whitespace)@77..78 " "
                  Token(SymArrow)@78..80 "->"
                  Context(Expr)@80..87
                    Token(Whitespace)@80..81 " "
                    Context(String)@81..86
                      Token(SymDoubleQuote)@81..82 "\""
                      StringToken(Text)@82..85 "baz"
                      StringToken(Delimiter)@85..86 "\""
                    Token(Whitespace)@86..87 " "
                Context(Branch)@87..100
                  Token(KWCase)@87..91 "case"
                  Context(Pattern)@91..93
                    Token(Whitespace)@91..92 " "
                    Token(SymUnderscore)@92..93 "_"
                  Token(Whitespace)@93..94 " "
                  Token(SymArrow)@94..96 "->"
                  Context(Expr)@96..100
                    Token(Whitespace)@96..97 " "
                    Context(String)@97..100
                      Token(SymDoubleQuote)@97..98 "\""
                      StringToken(Text)@98..99 "!"
                      StringToken(Delimiter)@99..100 "\"""##]],
    )
}
#[test]
fn parse_switch_sample() {
    check(
        r#"switch _ on case "a" -> #true case "e" -> #true case "i" -> #true case "o" -> #true case "u" -> #true case _   -> #false"#,
        expect![[r##"
            Context(Expr)@0..120
              Context(Switch)@0..120
                Token(KWSwitch)@0..6 "switch"
                Context(Expr)@6..9
                  Token(Whitespace)@6..7 " "
                  Token(SymUnderscore)@7..8 "_"
                  Token(Whitespace)@8..9 " "
                Token(KWOn)@9..11 "on"
                Context(Branch)@11..30
                  Token(Whitespace)@11..12 " "
                  Token(KWCase)@12..16 "case"
                  Context(Pattern)@16..20
                    Token(Whitespace)@16..17 " "
                    Context(String)@17..20
                      Token(SymDoubleQuote)@17..18 "\""
                      StringToken(Text)@18..19 "a"
                      StringToken(Delimiter)@19..20 "\""
                  Token(Whitespace)@20..21 " "
                  Token(SymArrow)@21..23 "->"
                  Context(Expr)@23..30
                    Token(Whitespace)@23..24 " "
                    Context(Constructor)@24..30
                      Token(SymHash)@24..25 "#"
                      Token(IdLower)@25..29 "true"
                      Context(Args)@29..30
                        Token(Whitespace)@29..30 " "
                Context(Branch)@30..48
                  Token(KWCase)@30..34 "case"
                  Context(Pattern)@34..38
                    Token(Whitespace)@34..35 " "
                    Context(String)@35..38
                      Token(SymDoubleQuote)@35..36 "\""
                      StringToken(Text)@36..37 "e"
                      StringToken(Delimiter)@37..38 "\""
                  Token(Whitespace)@38..39 " "
                  Token(SymArrow)@39..41 "->"
                  Context(Expr)@41..48
                    Token(Whitespace)@41..42 " "
                    Context(Constructor)@42..48
                      Token(SymHash)@42..43 "#"
                      Token(IdLower)@43..47 "true"
                      Context(Args)@47..48
                        Token(Whitespace)@47..48 " "
                Context(Branch)@48..66
                  Token(KWCase)@48..52 "case"
                  Context(Pattern)@52..56
                    Token(Whitespace)@52..53 " "
                    Context(String)@53..56
                      Token(SymDoubleQuote)@53..54 "\""
                      StringToken(Text)@54..55 "i"
                      StringToken(Delimiter)@55..56 "\""
                  Token(Whitespace)@56..57 " "
                  Token(SymArrow)@57..59 "->"
                  Context(Expr)@59..66
                    Token(Whitespace)@59..60 " "
                    Context(Constructor)@60..66
                      Token(SymHash)@60..61 "#"
                      Token(IdLower)@61..65 "true"
                      Context(Args)@65..66
                        Token(Whitespace)@65..66 " "
                Context(Branch)@66..84
                  Token(KWCase)@66..70 "case"
                  Context(Pattern)@70..74
                    Token(Whitespace)@70..71 " "
                    Context(String)@71..74
                      Token(SymDoubleQuote)@71..72 "\""
                      StringToken(Text)@72..73 "o"
                      StringToken(Delimiter)@73..74 "\""
                  Token(Whitespace)@74..75 " "
                  Token(SymArrow)@75..77 "->"
                  Context(Expr)@77..84
                    Token(Whitespace)@77..78 " "
                    Context(Constructor)@78..84
                      Token(SymHash)@78..79 "#"
                      Token(IdLower)@79..83 "true"
                      Context(Args)@83..84
                        Token(Whitespace)@83..84 " "
                Context(Branch)@84..102
                  Token(KWCase)@84..88 "case"
                  Context(Pattern)@88..92
                    Token(Whitespace)@88..89 " "
                    Context(String)@89..92
                      Token(SymDoubleQuote)@89..90 "\""
                      StringToken(Text)@90..91 "u"
                      StringToken(Delimiter)@91..92 "\""
                  Token(Whitespace)@92..93 " "
                  Token(SymArrow)@93..95 "->"
                  Context(Expr)@95..102
                    Token(Whitespace)@95..96 " "
                    Context(Constructor)@96..102
                      Token(SymHash)@96..97 "#"
                      Token(IdLower)@97..101 "true"
                      Context(Args)@101..102
                        Token(Whitespace)@101..102 " "
                Context(Branch)@102..120
                  Token(KWCase)@102..106 "case"
                  Context(Pattern)@106..108
                    Token(Whitespace)@106..107 " "
                    Token(SymUnderscore)@107..108 "_"
                  Token(Whitespace)@108..111 "   "
                  Token(SymArrow)@111..113 "->"
                  Context(Expr)@113..120
                    Token(Whitespace)@113..114 " "
                    Context(Constructor)@114..120
                      Token(SymHash)@114..115 "#"
                      Token(IdLower)@115..120 "false"
                      Context(Args)@120..120"##]],
    )
}

#[test]
fn parse_lambda() {
    check(
        "fun a b -> a * (3 - b)",
        expect![[r#"
            Context(Expr)@0..22
              Context(Lambda)@0..22
                Token(KWFun)@0..3 "fun"
                Context(Params)@3..10
                  Context(Pattern)@3..5
                    Token(Whitespace)@3..4 " "
                    Token(IdLower)@4..5 "a"
                  Token(Whitespace)@5..6 " "
                  Context(Pattern)@6..7
                    Token(IdLower)@6..7 "b"
                  Token(Whitespace)@7..8 " "
                  Token(SymArrow)@8..10 "->"
                Context(BinOp)@10..22
                  Token(Whitespace)@10..11 " "
                  Token(IdLower)@11..12 "a"
                  Token(Whitespace)@12..13 " "
                  Token(OpMul)@13..14 "*"
                  Token(Whitespace)@14..15 " "
                  Context(Parenthesised)@15..22
                    Token(SymLParen)@15..16 "("
                    Context(BinOp)@16..21
                      Token(Number)@16..17 "3"
                      Token(Whitespace)@17..18 " "
                      Token(OpSub)@18..19 "-"
                      Token(Whitespace)@19..20 " "
                      Token(IdLower)@20..21 "b"
                    Token(SymRParen)@21..22 ")""#]],
    )
}
