use crate::parse_expression;

use super::{expr::Expr, FromSyntaxElement, ToHIR};

// #[test]
// #[ignore = "module unimplemented"]
// fn parse_sample_file() {
//     let file = File::open("./sample.ren").unwrap();
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents).expect("Error reading file");

//     let parsed = parse_module(contents.as_str());
// }

#[test]
fn parse_sample_exprs() {
    let exprs = include_str!("./sample_expressions.ren")
        .split_terminator("\n\n")
        .filter_map(|line| {
            Expr::from_root_node(parse_expression(line).syntax()).map(|e| e.to_higher_ast())
            //.map(|expr| (line, expr))
        })
        .collect::<Vec<_>>();
    expect_test::expect_file!["./sample_expressions.ren.expected"].assert_debug_eq(&exprs);
}
