use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::syntax::{
    lexer::{Lexeme, Lexer},
    RenLang, SyntaxNode, Token, TokenType,
};

mod marker;
pub(crate) use marker::Marker;

pub(crate) struct Parser<'source> {
    lexer: Lexer<'source>,
    builder: GreenNodeBuilder<'static>,
    whitespace_token: Option<Lexeme<'source>>,
}

impl<'source> Parser<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
            whitespace_token: None,
        }
    }
    pub fn start(&mut self, label: &'static str) -> Marker {
        Marker::new(self.builder.checkpoint(), label)
    }
    pub fn bump(&mut self) {
        let (kind, text) = self.lexer.next().expect("Tried to bump at end of input");
        self.builder
            .token(RenLang::kind_to_raw(kind.into()), text.into());
    }
    pub fn bump_matching<T: Into<TokenType>>(&mut self, token: T) -> bool {
        if self.peek() == token.into() {
            self.bump();
            true
        } else {
            false
        }
    }
    pub fn parse(self) -> Parsed {
        Parsed {
            green_node: self.builder.finish(),
        }
    }
    pub fn bump_whitespace(&mut self) -> bool {
        if self.whitespace_token.is_some() {
            self.whitespace_token = None;
            true
        } else if let TokenType::Token(Token::Whitespace) = self.peek_internal() {
            self.bump();
            true
        } else {
            false
        }
    }
    pub fn peek(&mut self) -> TokenType {
        loop {
            let peek = self.peek_internal();
            match peek {
                TokenType::Token(Token::Whitespace) => {
                    self.whitespace_token = self.lexer.peek().into();
                    self.bump();
                }
                TokenType::Token(Token::Comment) => {
                    self.bump();
                }
                _ => {
                    return peek;
                }
            }
        }
    }
    fn peek_internal(&mut self) -> TokenType {
        self.lexer
            .peek()
            .map(|(ty, _)| ty)
            .unwrap_or(TokenType::None)
    }
}

pub struct Parsed {
    green_node: GreenNode,
}
impl Parsed {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
    pub fn into_ast(&self) -> ast::expr::Expr {
        todo!()
    }
}
