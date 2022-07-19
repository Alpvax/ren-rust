mod expression;
mod module;

#[test]
fn parse_nothing() {
    let parse = super::parse_module("");
    expect_test::expect![[r#"Context(Module)@0..0"#]].assert_eq(&parse.debug_tree())
}

#[test]
#[ignore]
fn parse_remote_reference_file() {
    if let Ok(input) = reqwest::blocking::get("https://raw.githubusercontent.com/ren-lang/compiler/dd75310b42fc34b04f3b40af27333a4a06f62d73/reference/syntax.ren").and_then(|r| r.text()) {
        // println!("Parsing:\n{}\n================================================================================", input);
        let parse = super::parse_module(input.as_str());
        expect_test::expect_file!["./syntax.ren.parsed"].assert_eq(&parse.debug_tree());
    } else {
        panic!("Failed reading from remote file");
    }
}
