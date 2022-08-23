use ::expect_test::{expect, Expect};

use crate::expr::{Expr, Pattern};

fn check_serialise<T>(to_ser: T, expected: Expect)
where
    T: ::serde::Serialize + std::fmt::Debug,
{
    expected.assert_eq(
        &serde_json::to_string_pretty(&to_ser)
            .expect(&format!("Error serialising {:?} to JSON", to_ser)),
    );
}
fn check_deserialise<T>(input: &'static str, expected: Expect)
where
    T: ::serde::Deserialize<'static> + std::fmt::Debug,
{
    expected.assert_debug_eq(
        &serde_json::from_str::<'static, T>(&input)
            .expect(&format!("Error deserialising {:?} from JSON", input)),
    );
}

mod literal {
    use crate::expr::Literal;

    use super::*;

    #[test]
    fn number_ser() {
        check_serialise(
            Literal::<Expr>::from(143),
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
    fn number_de() {
        check_deserialise::<Literal<Pattern>>(
            r#"
            [
              {
                "$": "Number"
              },
              143.0
            ]"#,
            expect![[r""]],
        );
    }

    #[test]
    fn simple_string() {
        check_serialise(
            Literal::<Expr>::from("Hello World"),
            expect![[r#"
                [
                  {
                    "$": "String"
                  },
                  [
                    [
                      {
                        "$": "Text"
                      },
                      "Hello World"
                    ]
                  ]
                ]"#]],
        )
    }

    #[test]
    fn string_with_escapes() {
        check_serialise(
            Literal::<Expr>::from("Hello\n\tWorld"),
            expect![[r#"
                [
                  {
                    "$": "String"
                  },
                  [
                    [
                      {
                        "$": "Text"
                      },
                      "Hello\n\tWorld"
                    ]
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
                    [
                      {
                        "$": "Text"
                      },
                      "Hello\n"
                    ],
                    [
                      {
                        "$": "Value"
                      },
                      [
                        {
                          "$": "Lit",
                          "type": [
                            {
                              "$": "Hole"
                            }
                          ],
                          "span": null,
                          "comment": []
                        },
                        [
                          {
                            "$": "String"
                          },
                          [
                            [
                              {
                                "$": "Text"
                              },
                              "\tworld \\${text}"
                            ]
                          ]
                        ]
                      ]
                    ]
                  ]
                ]"#]],
        )
    }

    #[test]
    fn array() {
        check_serialise(
            Expr::literal(vec![Expr::literal(3), Expr::literal(13)]),
            expect![[r#"
                [
                  {
                    "$": "Lit",
                    "type": [
                      {
                        "$": "Hole"
                      }
                    ],
                    "span": null,
                    "comment": []
                  },
                  [
                    {
                      "$": "Array"
                    },
                    [
                      [
                        {
                          "$": "Lit",
                          "type": [
                            {
                              "$": "Hole"
                            }
                          ],
                          "span": null,
                          "comment": []
                        },
                        [
                          {
                            "$": "Number"
                          },
                          3.0
                        ]
                      ],
                      [
                        {
                          "$": "Lit",
                          "type": [
                            {
                              "$": "Hole"
                            }
                          ],
                          "span": null,
                          "comment": []
                        },
                        [
                          {
                            "$": "Number"
                          },
                          13.0
                        ]
                      ]
                    ]
                  ]
                ]"#]],
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
