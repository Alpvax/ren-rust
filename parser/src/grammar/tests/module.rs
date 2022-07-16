use crate::parse_module;
use expect_test::expect;

fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse_module(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[test]
fn parse_import() {
    check(r#"import "path""#, expect![[r#"
        Context(Module)@0..12
          Context(Imports)@0..12
            Context(Import)@0..12
              Token(KWImport)@0..6 "import"
              Context(String)@6..12
                Token(DoubleQuote)@6..7 "\""
                StringToken(Text)@7..11 "path"
                StringToken(Delimiter)@11..12 "\"""#]])
}

#[test]
fn parse_import_as() {
    check(r#"import "path" as Name"#, expect![[r#"
        Context(Module)@0..18
          Context(Imports)@0..18
            Context(Import)@0..18
              Token(KWImport)@0..6 "import"
              Context(String)@6..12
                Token(DoubleQuote)@6..7 "\""
                StringToken(Text)@7..11 "path"
                StringToken(Delimiter)@11..12 "\""
              Token(KWAs)@12..14 "as"
              Context(NameSpace)@14..18
                Token(Namespace)@14..18 "Name""#]])
}

#[test]
fn parse_import_as_multi() {
    check(r#"import "path" as Name.Space"#, expect![[r#"
        Context(Module)@0..24
          Context(Imports)@0..24
            Context(Import)@0..24
              Token(KWImport)@0..6 "import"
              Context(String)@6..12
                Token(DoubleQuote)@6..7 "\""
                StringToken(Text)@7..11 "path"
                StringToken(Delimiter)@11..12 "\""
              Token(KWAs)@12..14 "as"
              Context(NameSpace)@14..24
                Token(Namespace)@14..18 "Name"
                Token(Period)@18..19 "."
                Token(Namespace)@19..24 "Space""#]])
}

#[test]
fn parse_import_exposing() {
    check(r#"import "path" exposing {foo}"#, expect![[r#"
        Context(Module)@0..25
          Context(Imports)@0..25
            Context(Import)@0..25
              Token(KWImport)@0..6 "import"
              Context(String)@6..12
                Token(DoubleQuote)@6..7 "\""
                StringToken(Text)@7..11 "path"
                StringToken(Delimiter)@11..12 "\""
              Token(KWExposing)@12..20 "exposing"
              Token(CurlyOpen)@20..21 "{"
              Context(ExposingBlock)@21..25
                Token(VarName)@21..24 "foo"
                Token(CurlyClose)@24..25 "}""#]])
}

#[test]
fn parse_import_exposing_multi() {
    check(r#"import "path" exposing {foo, bar}"#, expect![[r#"
        Context(Module)@0..29
          Context(Imports)@0..29
            Context(Import)@0..29
              Token(KWImport)@0..6 "import"
              Context(String)@6..12
                Token(DoubleQuote)@6..7 "\""
                StringToken(Text)@7..11 "path"
                StringToken(Delimiter)@11..12 "\""
              Token(KWExposing)@12..20 "exposing"
              Token(CurlyOpen)@20..21 "{"
              Context(ExposingBlock)@21..29
                Token(VarName)@21..24 "foo"
                Token(Comma)@24..25 ","
                Token(VarName)@25..28 "bar"
                Token(CurlyClose)@28..29 "}""#]])
}

#[test]
fn parse_full_import() {
    check(r#"import "path" as Name.Space exposing {foo, bar}"#, expect![[r#"
        Context(Module)@0..41
          Context(Imports)@0..41
            Context(Import)@0..41
              Token(KWImport)@0..6 "import"
              Context(String)@6..12
                Token(DoubleQuote)@6..7 "\""
                StringToken(Text)@7..11 "path"
                StringToken(Delimiter)@11..12 "\""
              Token(KWAs)@12..14 "as"
              Context(NameSpace)@14..24
                Token(Namespace)@14..18 "Name"
                Token(Period)@18..19 "."
                Token(Namespace)@19..24 "Space"
              Token(KWExposing)@24..32 "exposing"
              Token(CurlyOpen)@32..33 "{"
              Context(ExposingBlock)@33..41
                Token(VarName)@33..36 "foo"
                Token(Comma)@36..37 ","
                Token(VarName)@37..40 "bar"
                Token(CurlyClose)@40..41 "}""#]])
}

#[test]
fn multiple_imports() {
    check(r#"import "path" as Name.Space
    import "./path2" exposing {foo, bar}"#, expect![[r#"
        Context(Module)@0..56
          Context(Imports)@0..56
            Context(Import)@0..24
              Token(KWImport)@0..6 "import"
              Context(String)@6..12
                Token(DoubleQuote)@6..7 "\""
                StringToken(Text)@7..11 "path"
                StringToken(Delimiter)@11..12 "\""
              Token(KWAs)@12..14 "as"
              Context(NameSpace)@14..24
                Token(Namespace)@14..18 "Name"
                Token(Period)@18..19 "."
                Token(Namespace)@19..24 "Space"
            Context(Import)@24..56
              Token(KWImport)@24..30 "import"
              Context(String)@30..39
                Token(DoubleQuote)@30..31 "\""
                StringToken(Text)@31..38 "./path2"
                StringToken(Delimiter)@38..39 "\""
              Token(KWExposing)@39..47 "exposing"
              Token(CurlyOpen)@47..48 "{"
              Context(ExposingBlock)@48..56
                Token(VarName)@48..51 "foo"
                Token(Comma)@51..52 ","
                Token(VarName)@52..55 "bar"
                Token(CurlyClose)@55..56 "}""#]])
}