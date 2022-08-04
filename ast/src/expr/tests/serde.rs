use expect_test::{expect, Expect};

use crate::expr::Expr;

fn check_serialise<T>(to_ser: T, expected: Expect)
where
    T: serde::Serialize + std::fmt::Debug,
{
    expected.assert_eq(
        &serde_json::to_string_pretty(&to_ser)
            .expect(&format!("Error serialising {:?} to JSON", to_ser)),
    )
}

mod literal {
    use crate::expr::Literal;

    use super::*;

    #[test]
    fn number() {
        check_serialise(
            Literal::<()>::from(143),
            expect![[r#"
            [
              {
                "$": "Number"
              },
              143.0
            ]"#]],
        );
    }

    #[test]
    fn simple_string() {
        check_serialise(
            Literal::<()>::from("Hello World"),
            expect![[r#"
                [
                  {
                    "$": "String"
                  },
                  [
                    {
                      "$": "Text"
                    },
                    "Hello World"
                  ]
                ]"#]],
        )
    }

    #[test]
    fn string_with_escapes() {
        check_serialise(
            Literal::<()>::from("Hello\n\tWorld"),
            expect![[r#"
                [
                  {
                    "$": "String"
                  },
                  [
                    {
                      "$": "Text"
                    },
                    "Hello\n\tWorld"
                  ]
                ]"#]],
        )
    }

    #[test]
    fn nested_string() {
        check_serialise(
            Literal::<Expr>::LStr(vec![
                "Hello\n".into(),
                Expr::literal("\tworld \\${text}").into(),
            ]),
            expect![[r#"
                [
                  {
                    "$": "String"
                  },
                  [
                    {
                      "$": "Text"
                    },
                    "Hello\n"
                  ],
                  [
                    {
                      "$": "Lit"
                    },
                    [
                      {
                        "$": "String"
                      },
                      [
                        {
                          "$": "Text"
                        },
                        "\tworld \\${text}"
                      ]
                    ]
                  ]
                ]"#]],
        )
    }

    #[test]
    fn array() {
        check_serialise(
            Literal::Array(vec![Expr::literal("foo"), Expr::literal("bar")]),
            expect![[r#"
                [
                    {
                        "$": "Array"
                    },
                    [
                        [
                            {
                                "$": "Lit"
                            },
                            [
                                {
                                    "$": "String"
                                },
                                "foo"
                            ]
                        ],
                        [
                            {
                                "$": "Lit"
                            },
                            [
                                {
                                    "$": "String"
                                },
                                "bar"
                            ]
                        ]
                    ]
                ]""#]],
        )
    }

    // #[test]
    // fn record() {
    //     check_serialise(
    //         Literal::<()>::from({foo, bar: baz}),
    //         expect![[r#"
    //             Context(Pattern)@0..15
    //               Context(Record)@0..15
    //                 Token(CurlyOpen)@0..1 "{"
    //                 Context(Field)@1..5
    //                   Token(VarName)@1..4 "foo"
    //                   Token(Comma)@4..5 ","
    //                 Context(Field)@5..14
    //                   Token(Whitespace)@5..6 " "
    //                   Token(VarName)@6..9 "bar"
    //                   Token(Colon)@9..10 ":"
    //                   Token(Whitespace)@10..11 " "
    //                   Token(VarName)@11..14 "baz"
    //                 Token(CurlyClose)@14..15 "}""#]],
    //     )
    //}
}
