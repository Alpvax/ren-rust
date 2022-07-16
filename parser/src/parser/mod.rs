use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::lexer::{Context, Lexer, RenLang, SyntaxNode, SyntaxPart, Token, TokenType};

mod expression;

pub struct Parser<'source> {
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
    // pub fn parse_module(mut self) -> Parse {
    //     self.start_node(Context::Module);
    //     //TODO: implement parser
    //     self.finish_node();
    //     Parse { green_node: self.builder.finish() }
    // }
    // pub fn parse_expression(mut self) -> Parse {
    //     self.start_node(Context::Module);
    //     //TODO: implement parser
    //     self.finish_node();
    //     Parse { green_node: self.builder.finish() }
    // }
    pub fn parse(mut self) -> Parse {
        self.start_node(Context::Module);
        //TODO: implement parser
        expression::expr(&mut self);

        self.finish_node();
        Parse {
            green_node: self.builder.finish(),
        }
    }
    fn start_node<K: Into<SyntaxPart>>(&mut self, kind: K) {
        self.builder.start_node(RenLang::kind_to_raw(kind.into()));
    }
    fn start_node_at<K: Into<SyntaxPart>>(&mut self, checkpoint: rowan::Checkpoint, kind: K) {
        self.builder
            .start_node_at(checkpoint, RenLang::kind_to_raw(kind.into()));
    }
    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
    fn peek(&mut self) -> TokenType {
        self.peek_non_trivia(false)
    }
    fn peek_non_trivia(&mut self, emit_whitespace: bool) -> TokenType {
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
    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().expect("Tried to bump at end of input");
        self.builder
            .token(RenLang::kind_to_raw(kind.into()), text.into());
    }
    fn checkpoint(&self) -> rowan::Checkpoint {
        self.builder.checkpoint()
    }
}

pub struct Parse {
    green_node: GreenNode,
}
impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = Parser::new(input).parse();
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Context(Module)@0..0"#]])
    }

    #[test]
    fn parse_commented_expr() {
        check(
            "
1 + 2 // Not applied as a single term
* 3 // = 6
/
(5 - -2) // 5 - (-2) = 6",
            expect![[r#"
                Context(Module)@0..64
                  Context(BinOp)@0..64
                    Token(Number)@0..1 "1"
                    Token(OpAdd)@1..2 "+"
                    Context(BinOp)@2..64
                      Context(BinOp)@2..42
                        Token(Number)@2..3 "2"
                        Token(Comment)@3..34 "// Not applied as a s ..."
                        Token(OpMul)@34..35 "*"
                        Token(Number)@35..36 "3"
                        Token(Comment)@36..42 "// = 6"
                      Token(OpDiv)@42..43 "/"
                      Token(ParenOpen)@43..44 "("
                      Context(BinOp)@44..48
                        Token(Number)@44..45 "5"
                        Token(OpSub)@45..46 "-"
                        Context(PrefixOp)@46..48
                          Token(OpSub)@46..47 "-"
                          Token(Number)@47..48 "2"
                      Token(ParenClose)@48..49 ")"
                      Token(Comment)@49..64 "// 5 - (-2) = 6""#]],
        )
    }
}
