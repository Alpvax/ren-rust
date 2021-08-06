mod template_literal {
    use crate::token::string::TemplateLiteralToken as T;
    use logos::{Logos};

    #[test]
    fn ending_expression() {
        assert_eq!(
            T::lexer("`A template literal containing an ${expression}`").into_iter().collect::<Vec<_>>(),
            vec![
                T::Delimiter,
                T::Text("A template literal containing an ".to_owned()),
                T::ExprStart,
                T::Text("expression}".to_owned()), //TODO: switch mode
                T::Delimiter,
            ]
        );
    }
}
mod double_quoted_string {
    use crate::token::string::DoubleStringToken as T;
    use logos::{Logos};

    #[test]
    fn escapes() {
        assert_eq!(
            T::lexer(r#""A double quoted string literal\n\tcontaining \"escapes\"""#).into_iter().collect::<Vec<_>>(),
            vec![
                T::Delimiter,
                T::Text("A double quoted string literal".to_owned()),
                T::Escape('n'),
                T::Escape('t'),
                T::Text("containing ".to_owned()),
                T::Escape('"'),
                T::Text("escapes".to_owned()),
                T::Escape('"'),
                T::Delimiter,
            ]
        );
    }
    #[test]
    fn invalid_escape() {
        assert_eq!(
            T::lexer(r#""A double quoted string literal containing an invalid escape: \'""#).into_iter().collect::<Vec<_>>(),
            vec![
                T::Delimiter,
                T::Text("A double quoted string literal containing an invalid escape: ".to_owned()),
                T::Error,
                T::Text("'".to_owned()),
                T::Delimiter,
            ]
        );
    }
}
mod single_quoted_string {
    use crate::token::string::SingleStringToken as T;
    use logos::{Logos};

    #[test]
    fn escapes() {
        assert_eq!(
            T::lexer(r"'A single quoted string literal\n\tcontaining \'escapes\''").into_iter().collect::<Vec<_>>(),
            vec![
                T::Delimiter,
                T::Text("A single quoted string literal".to_owned()),
                T::Escape('n'),
                T::Escape('t'),
                T::Text("containing ".to_owned()),
                T::Escape('\''),
                T::Text("escapes".to_owned()),
                T::Escape('\''),
                T::Delimiter,
            ]
        );
    }
    #[test]
    fn invalid_escape() {
        assert_eq!(
            T::lexer(r#"'A single quoted string literal containing an invalid escape: \"'"#).into_iter().collect::<Vec<_>>(),
            vec![
                T::Delimiter,
                T::Text("A single quoted string literal containing an invalid escape: ".to_owned()),
                T::Error,
                T::Text("\"".to_owned()),
                T::Delimiter,
            ]
        );
    }
}
