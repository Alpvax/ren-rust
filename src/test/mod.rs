#[test]
#[cfg(feature = "reqwest")]
fn parse_remote_reference_file() {
    let parsed = super::parse_remote_file("https://raw.githubusercontent.com/ren-lang/compiler/dd75310b42fc34b04f3b40af27333a4a06f62d73/reference/syntax.ren").expect("Failed reading from remote file");
    expect_test::expect_file!["./syntax.ren.parsed"].assert_eq(&parsed.debug_tree());
}
