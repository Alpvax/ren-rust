use super::{
    helper::*,
    parse_declaration, parse_toplevel_declaration,
    DeclarationError::{self, *},
    Lexer, Token,
};
use crate::ast::declaration::{Definition, Visibility};
use crate::ast::expression::{Expression, Literal, Pattern};
use crate::ast::Declaration;

make_test_fn!(test_dec<Declaration, DeclarationError> = parse_toplevel_declaration);
make_test_fn!(test_sub_dec<Declaration, DeclarationError> = parse_declaration);

#[test]
fn parse_empty() {
    assert_err(test_dec(""), NoTokens, 0);
    assert_err(test_sub_dec(""), NoFunLet, 0);
}

fn fun_dec(
    public: bool,
    name: &str,
    args: Vec<Pattern>,
    bindings: Vec<Declaration>,
    body: Literal,
) -> Declaration {
    Declaration::new(
        Vec::new(),
        if public {
            Visibility::Public
        } else {
            Visibility::Private
        },
        Definition::Function {
            name: name.to_owned(),
            args,
        },
        bindings,
        Expression::Literal(body),
    )
}
fn let_dec(public: bool, name: Pattern, bindings: Vec<Declaration>, body: Literal) -> Declaration {
    Declaration::new(
        Vec::new(),
        if public {
            Visibility::Public
        } else {
            Visibility::Private
        },
        Definition::Variable { name },
        bindings,
        Expression::Literal(body),
    )
}

#[test]
fn parse_simple() {
    assert_eq!(
        ok_remaining(test_dec("fun f = _ => 3"), 0),
        fun_dec(
            false,
            "f",
            vec![Pattern::Wildcard(None)],
            Vec::new(),
            Literal::Number(3.0),
        ),
    );
    assert_eq!(
        ok_remaining(test_dec("fun f = a b => 3"), 0),
        fun_dec(
            false,
            "f",
            vec![Pattern::Name("a".to_owned()), Pattern::Name("b".to_owned())],
            Vec::new(),
            Literal::Number(3.0),
        ),
    );
    assert_eq!(
        ok_remaining(test_dec("let x = 3"), 0),
        let_dec(
            false,
            Pattern::Name("x".to_owned()),
            Vec::new(),
            Literal::Number(3.0),
        ),
    );
    assert_eq!(
        ok_remaining(test_dec("let {x: [a, _ignore, b]} = 3"), 0),
        let_dec(
            false,
            Pattern::ObjectDestructure(vec![(
                "x".to_owned(),
                Some(Pattern::ArrayDestructure(vec![
                    Pattern::Name("a".to_owned()),
                    Pattern::Wildcard(Some("ignore".to_owned())),
                    Pattern::Name("b".to_owned()),
                ]))
            )]),
            Vec::new(),
            Literal::Number(3.0),
        ),
    );
}

#[test]
fn parse_block() {
    let l1 = let_dec(
        false,
        Pattern::Name("a".to_owned()),
        Vec::new(),
        Literal::Boolean(true),
    );
    let l2 = let_dec(
        false,
        Pattern::Name("b".to_owned()),
        Vec::new(),
        Literal::Boolean(false),
    );
    let f1 = fun_dec(
        false,
        "f",
        vec![Pattern::Name("x".to_owned())],
        Vec::new(),
        Literal::Number(12.0),
    );
    let f2 = fun_dec(
        false,
        "g",
        vec![Pattern::Name("y".to_owned())],
        Vec::new(),
        Literal::String("baz".to_owned()),
    );

    assert_eq!(
        ok_remaining(
            test_dec(
                "fun foo = p => {
  let a = true
  ret 0xff
}"
            ),
            0
        ),
        fun_dec(
            false,
            "foo",
            vec![Pattern::Name("p".to_owned())],
            vec![l1.clone()],
            Literal::Number(255.0)
        ),
    );
    assert_eq!(
        ok_remaining(
            test_dec(
                "fun foo = p => {
  let a = true
  fun f = x => 12
  ret 0xff
}"
            ),
            0
        ),
        fun_dec(
            false,
            "foo",
            vec![Pattern::Name("p".to_owned())],
            vec![l1.clone(), f1.clone()],
            Literal::Number(255.0)
        ),
    );
    assert_eq!(
        ok_remaining(
            test_dec(
                "let bar = {
  let a = true
  let b = false
  fun f = x => 12
  fun g = y => \"baz\"
  ret 'Hello World'
}"
            ),
            0
        ),
        let_dec(
            false,
            Pattern::Name("bar".to_owned()),
            vec![l1.clone(), l2.clone(), f1.clone(), f2.clone()],
            Literal::String("Hello World".to_owned())
        ),
    );
}

#[test]
fn parse_visibility() {
    assert_eq!(
        ok_remaining(test_dec("pub fun f = _ => 3"), 0),
        fun_dec(
            true,
            "f",
            vec![Pattern::Wildcard(None)],
            Vec::new(),
            Literal::Number(3.0)
        ),
    );
    assert_eq!(
        ok_remaining(test_dec("fun f = _ => 3"), 0),
        fun_dec(
            false,
            "f",
            vec![Pattern::Wildcard(None)],
            Vec::new(),
            Literal::Number(3.0)
        ),
    );
    assert_eq!(
        ok_remaining(test_dec("pub let x = 'foo'"), 0),
        let_dec(
            true,
            Pattern::Name("x".to_owned()),
            Vec::new(),
            Literal::String("foo".to_owned())
        ),
    );
    assert_eq!(
        ok_remaining(test_dec("let x = 'foo'"), 0),
        let_dec(
            false,
            Pattern::Name("x".to_owned()),
            Vec::new(),
            Literal::String("foo".to_owned())
        ),
    );
}
