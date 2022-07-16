use expect_test::expect;

use super::parse_expression;

fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse_expression(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

mod expression;

#[test]
fn parse_nothing() {
    check("", expect![[r#"Context(Module)@0..0"#]])
}

#[test]
fn parse_commented_expr() {
    check(
        "
1 + 2 // Not applied as a single term
* 3 // = 6
/
(5 - -2) // 5 - (-2) = 6",
        expect![[r#"
                Context(Module)@0..64
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
