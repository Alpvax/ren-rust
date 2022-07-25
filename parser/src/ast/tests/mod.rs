use crate::{parse_expression, syntax::Context};

use super::{FromSyntaxElement, expr::*};

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
    include_str!("./sample_expressions.ren").split_terminator("\n").filter_map(|line| {
        if line.trim().len() < 1 {
            return None;
        }
        let parsed = parse_expression(line);
        Expr::from_node(Context::Expr, parsed.syntax()).map(|expr| (line, expr))
    }).zip([
        r#"ELambda(LambdaExpr(Context(Lambda)@0..152))"#,
        "TODO",
        "TODO",
        "TODO",
        "TODO",
    ]).for_each(|((line, expr), expected)| {
        println!("line = \"{}\";\nexpr = \"{:?}\"\nexpected = {:?}", line, expr, expected);//XXX
        assert_eq!(format!("{:?}", expr), expected, "parsing line \"{}\"", line)
    });

}
