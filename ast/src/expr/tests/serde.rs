use ::expect_test::expect;

use crate::expr::{Expr, Pattern};

macro_rules! check_serde {
    ($name:ident: $typ:ty = $val:expr => $expect_ser:expr, $expect_de:expr $(,)?) => {
        mod $name {
            use super::*;
            #[test]
            fn ser() {
                let value = <$typ>::from($val);
                $expect_ser.assert_eq(
                    &serde_json::to_string_pretty(&value)
                        .expect(&format!("Error serialising {:?} to JSON", value)),
                );
            }
            #[test]
            fn de() {
                let input = $expect_ser.data();
                $expect_de.assert_debug_eq(
                    &serde_json::from_str::<'static, $typ>(&input)
                        .expect(&format!("Error deserialising {:?} from JSON", input)),
                );
            }
        }
    };
}

mod literal {
    use super::*;
    use crate::expr::Literal;
    /// Implement for unit to allow testing literals with neither patterns nor expressions
    impl crate::ASTLiteralType for () {}

    macro_rules! check_serde_literal {
        ($name:ident: $typ:ty = $val:expr => $expect_ser:expr, $expect_de:expr $(,)?) => {
            check_serde! {$name: Literal<$typ> = $val => $expect_ser, $expect_de}
        };
        ($name:ident = $val:expr => $expect_ser:expr, $expect_de:expr $(,)?) => {
            check_serde_literal! {$name: Literal<()> = $val => $expect_ser, $expect_de}
        };
    }

    check_serde_literal! {
        number = 143 =>
        expect![[r#"
            [
              {
                "$": "Number"
              },
              143.0
            ]"#]],
        expect![[r"
            Number(
                143.0,
            )
        "]]
    }

    mod string {
        use super::*;

        check_serde_literal! {
            simple: Literal<()> = "Hello World" =>
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
            expect![[r#"
                LStr(
                    [
                        Text(
                            "Hello World",
                        ),
                    ],
                )
            "#]],
        }

        check_serde_literal! {
            with_escapes = "Hello\n\tWorld" =>
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
            expect![[r#"
                LStr(
                    [
                        Text(
                            "Hello\n\tWorld",
                        ),
                    ],
                )
            "#]],
        }

        check_serde_literal! {
            nested: Expr =
                Literal::<Expr>::LStr(vec![
                    "Hello\n".into(),
                    Expr::literal("\tworld \\${text}").into(),
                ]) =>
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
            expect![[r#"
                LStr(
                    [
                        Text(
                            "Hello\n",
                        ),
                        Value(
                            Literal(
                                Meta {
                                    typ: Hole,
                                    span: (),
                                    comment: [],
                                },
                                LStr(
                                    [
                                        Text(
                                            "\tworld \\${text}",
                                        ),
                                    ],
                                ),
                            ),
                        ),
                    ],
                )
            "#]],
        }
    }

    check_serde_literal! {
        array: Expr = vec![Expr::literal(3), Expr::literal(13)] =>
        expect![[r#"
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
            ]"#]],
        expect![[r#"
            Array(
                [
                    Literal(
                        Meta {
                            typ: Hole,
                            span: (),
                            comment: [],
                        },
                        Number(
                            3.0,
                        ),
                    ),
                    Literal(
                        Meta {
                            typ: Hole,
                            span: (),
                            comment: [],
                        },
                        Number(
                            13.0,
                        ),
                    ),
                ],
            )
        "#]],
    }

    check_serde_literal! {
        record: Pattern = vec![("foo", Pattern::Any), ("bar", Pattern::Var("baz".to_string()))] =>
        expect![[r#"
            [
              {
                "$": "Record"
              },
              [
                [
                  "foo",
                  [
                    {
                      "$": "Any"
                    }
                  ]
                ],
                [
                  "bar",
                  [
                    {
                      "$": "Var"
                    },
                    "baz"
                  ]
                ]
              ]
            ]"#]],
        expect![[r#"
            Record(
                [
                    (
                        "foo",
                        Any,
                    ),
                    (
                        "bar",
                        Var(
                            "baz",
                        ),
                    ),
                ],
            )
        "#]],
    }
}
