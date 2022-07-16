use super::super::check;
use expect_test::expect;

#[test]
fn parse_number() {
    check(
        "143",
        expect![[r#"
        Context(Module)@0..3
          Token(Number)@0..3 "143""#]],
    )
}

#[test]
fn parse_varname() {
    check(
        "varName1",
        expect![[r#"
        Context(Module)@0..8
          Token(VarName)@0..8 "varName1""#]],
    )
}

#[test]
fn parse_simple_binop() {
    check(
        "2+3",
        expect![[r#"
            Context(Module)@0..3
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
            Context(Module)@0..7
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
            Context(Module)@0..2
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
            Context(Module)@0..4
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
        Context(Module)@0..11
          Context(BinOp)@0..11
            Token(Number)@0..1 "1"
            Token(OpAdd)@1..2 "+"
            Context(BinOp)@2..11
              Context(BinOp)@2..5
                Token(Number)@2..3 "2"
                Token(OpMul)@3..4 "*"
                Token(Number)@4..5 "3"
              Token(OpDiv)@5..6 "/"
              Token(ParenOpen)@6..7 "("
              Context(BinOp)@7..10
                Token(Number)@7..8 "5"
                Token(OpSub)@8..9 "-"
                Token(Number)@9..10 "2"
              Token(ParenClose)@10..11 ")""#]],
    )
}
