use expect_test::expect;

fn check(input: &str, expected_tree: expect_test::Expect) {
    let mut p = crate::parser::Parser::new(input);
    super::super::parse_type(&mut p);
    let parse = p.parse();
    expected_tree.assert_eq(&parse.debug_tree());
}

#[test]
fn parse_any() {
    check(
        "*",
        expect![[r#"
            Context(Type)@0..1
              Token(OpMul)@0..1 "*""#]],
    );
}
