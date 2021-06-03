use super::{
    helper::*,
    parse_declaration, parse_toplevel_declaration,
    DeclarationError::{self, *},
    Lexer, Token,
};
use crate::ast::Declaration;

make_test_fn!(test_dec<Declaration, DeclarationError> = parse_toplevel_declaration);
make_test_fn!(test_sub_dec<Declaration, DeclarationError> = parse_declaration);

#[test]
fn parse_empty() {
    assert_err(test_dec(""), NoTokens, 0);
    assert_err(test_sub_dec(""), NoFunLet, 0);
}
