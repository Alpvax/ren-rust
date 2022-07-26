use smol_str::SmolStr;

use crate::syntax::{Context, SyntaxNode, Token};

use super::{
    extensions::{SyntaxIterator, SyntaxNodeExtension},
    FromSyntaxElement, SyntaxToken,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    PAny,
    PNum(PNumber),
    PStr(PString),
    PRec(PRecord),
    PArr(PArray),
    PCon(PConstructor),
    PTyp(PType),
    PVar(PVar),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PNumber(SyntaxToken);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PString(SyntaxNode);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PRecord(SyntaxNode);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PArray(SyntaxNode);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PConstructor(SyntaxNode);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PType(SyntaxNode);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PVar(SyntaxToken);

impl FromSyntaxElement for Pattern {
    fn from_token(token_type: Token, token: SyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        match token_type {
            Token::Placeholder => Some(Self::PAny),
            Token::Number => Some(Self::PNum(PNumber(token))),
            Token::VarName => Some(Self::PVar(PVar(token))),
            _ => None,
        }
    }

    fn from_node(context: Context, node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match context {
            Context::String => Some(Self::PStr(PString(node))),
            Context::Record => Some(Self::PRec(PRecord(node))),
            Context::Array => Some(Self::PArr(PArray(node))),
            Context::Constructor => Some(Self::PCon(PConstructor(node))),
            Context::TypeMatch => Some(Self::PTyp(PType(node))),
            _ => None,
        }
    }
}

impl PNumber {
    pub fn value(&self) -> Option<f64> {
        self.0.text().parse().ok()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringSegment {
    Text(SmolStr),
    Pat(Pattern),
}
impl PString {
    pub fn parts(&self) -> Vec<StringSegment> {
        let (mut parts, text) = self
            .0
            .children_with_tokens()
            .filter_map(|e| {
                use crate::syntax::{StringToken, SyntaxPart};
                match e.kind() {
                    SyntaxPart::StringToken(StringToken::Text)
                    | SyntaxPart::StringToken(StringToken::Escape) => e
                        .into_token()
                        .map(|t| StringSegment::Text(SmolStr::new(t.text()))),
                    SyntaxPart::Context(Context::Expr) => e
                        .into_node()
                        .and_then(Pattern::from_root_node)
                        .map(|expr| StringSegment::Pat(expr)),
                    _ => None,
                }
            })
            .fold((Vec::new(), Vec::new()), |(mut acc, mut text), part| {
                match part {
                    StringSegment::Text(txt) => {
                        text.push(txt);
                    }
                    pat_segment => {
                        if text.len() > 0 {
                            acc.push(StringSegment::Text(SmolStr::new(text.join(""))));
                            text.clear();
                        }
                        acc.push(pat_segment);
                    }
                }
                (acc, text)
            });
        if text.len() > 0 {
            parts.push(StringSegment::Text(SmolStr::new(text.join(""))));
        }
        parts
    }
}

impl PRecord {
    pub fn fields(&self) -> Vec<(String, Option<Pattern>)> {
        self.0
            .children()
            .filter_map(|field_node| {
                let mut iter = field_node.children_with_tokens().skip_trivia();
                iter.find(|n| n.kind() == Token::VarName.into())
                    .map(|n| n.into_token().unwrap().text().to_string())
                    .and_then(|name| {
                        Some((name.clone(), iter.last().and_then(Pattern::from_element)))
                    })
            })
            .collect()
    }
}

impl PArray {
    pub fn items(&self) -> Vec<Pattern> {
        self.0.map_children().collect()
    }
}

impl PConstructor {
    pub fn get_tag(&self) -> Option<SmolStr> {
        self.0
            .find_token(Token::VarName)
            .map(|t| SmolStr::new(t.text()))
    }
    pub fn get_args(&self) -> Vec<Pattern> {
        self.0
            .find_node(Context::Args)
            .map_or(Vec::default(), |node| {
                node.children_with_tokens()
                    .filter_map(Pattern::from_element)
                    .collect()
            })
    }
}

impl PType {
    pub fn type_name(&self) -> Option<SmolStr> {
        self.0.child_tokens().skip_trivia().find_map(|token| {
            if token.kind() == Token::Namespace.into() {
                Some(SmolStr::new(token.text()))
            } else {
                None
            }
        })
    }
    pub fn binding(&self) -> Option<Pattern> {
        self.0.children().last().and_then(Pattern::from_root_node)
    }
}

impl PVar {
    pub fn name(&self) -> SmolStr {
        SmolStr::new(self.0.text())
    }
}
