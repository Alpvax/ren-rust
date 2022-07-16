use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::syntax::{lexer::Lexer, RenLang, SyntaxNode, SyntaxPart, Token, TokenType};

// mod marker;
// use marker::Marker;

pub(crate) struct Parser<'source> {
    lexer: Lexer<'source>,
    builder: GreenNodeBuilder<'static>,
}

impl<'source> Parser<'source> {
    pub fn new(input: &'source str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
        }
    }
    // fn start(&mut self) -> Marker {
    //     Marker::new(self.builder.checkpoint())
    // }
    pub fn start_node<K: Into<SyntaxPart>>(&mut self, kind: K) {
        self.builder.start_node(RenLang::kind_to_raw(kind.into()));
    }
    pub fn start_node_at<K: Into<SyntaxPart>>(&mut self, checkpoint: rowan::Checkpoint, kind: K) {
        self.builder
            .start_node_at(checkpoint, RenLang::kind_to_raw(kind.into()));
    }
    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }
    pub fn checkpoint(&self) -> rowan::Checkpoint {
        self.builder.checkpoint()
    }
    pub fn bump(&mut self) {
        let (kind, text) = self.lexer.next().expect("Tried to bump at end of input");
        self.builder
            .token(RenLang::kind_to_raw(kind.into()), text.into());
    }
    pub fn parse(self) -> Parsed {
        Parsed {
            green_node: self.builder.finish(),
        }
    }
    pub fn peek(&mut self) -> TokenType {
        self.peek_non_trivia(false)
    }
    pub fn peek_non_trivia(&mut self, emit_whitespace: bool) -> TokenType {
        loop {
            let peek = self.peek_internal();
            match peek {
                TokenType::Token(Token::Whitespace) => {
                    if emit_whitespace {
                        return peek;
                    } else {
                        self.lexer.next()
                    };
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
