mod expression;
mod module;

#[test]
fn parse_nothing() {
    let parse = super::parse_module("");
    expect_test::expect![[r#"Context(Module)@0..0"#]].assert_eq(&parse.debug_tree())
}
