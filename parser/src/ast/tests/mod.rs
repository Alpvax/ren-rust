use crate::{parse_expression};

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
    include_str!("./sample_expressions.ren").split_terminator("\n\n").filter_map(|line| {
        println!("line = {}", line);//XXX
        let parsed = parse_expression(line);
        Expr::from_node(crate::syntax::Context::Expr, parsed.syntax()).map(|expr| (line, expr))
    }).zip([
        r#"LambdaExpr( [name], Application(Scoped([Str], join)), ["\\n", Array([...])]"#,
        "TODO",
        "TODO",
        "TODO",
        "TODO",
    ]).for_each(|((line, expr), expected)| assert_eq!(format!("{:?}", expr), expected, "parsing line \"{}\"", line));

}