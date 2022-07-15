use rowan::{GreenNode, GreenNodeBuilder, Language};

use crate::lexer::{Context, Lexer, RenLang, StringToken, SyntaxNode, SyntaxPart, Token};

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
    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
    fn is_string_token(&self) -> bool {
        self.lexer.is_string_token()
    }
    fn peek(&mut self) -> SyntaxPart {
        self.lexer
            .peek()
            .map(|(kind, _)| kind)
            .unwrap_or(SyntaxPart::EOF)
    }
    fn peek_token(&mut self) -> Option<Token> {
        match self.peek() {
            SyntaxPart::Token(tok) => Some(tok),
            _ => None,
        }
    }
    fn peek_str_token(&mut self) -> Option<StringToken> {
        match self.peek() {
            SyntaxPart::StringToken(tok) => Some(tok),
            _ => None,
        }
    }
    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().expect("Tried to bump at end of input");
        self.builder.token(RenLang::kind_to_raw(kind), text.into());
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
}
