use logos::{Lexer, Logos};

use crate::token::{
    expression::ExpressionToken,
    string::TemplateLiteralToken as T,
};

#[test]
fn begin_template() {
    let mut lex = ExpressionToken::lexer("`A template literal`");
    assert_eq!(lex.next(), Some(ExpressionToken::TemplateStart));
    let mut template_lex: Lexer<T> = lex.morph();
    assert_eq!(
        template_lex.into_iter().collect::<Vec<_>>(),
        vec![
            //T::Delimiter,
            T::Text("A template literal".to_owned()),
            T::Delimiter,
        ]
    );
}
