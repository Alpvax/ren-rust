use std::marker::PhantomData;

use either::Either;
use smol_str::SmolStr;

use crate::syntax::{Context, Token};

use super::{
    extensions::{SyntaxIterator, SyntaxNodeExtension},
    FromSyntaxElement, HigherASTWithVar, SyntaxNode, SyntaxToken, ToHIR,
};

type HigherLiteral<T> = higher_ast::Literal<T>;

pub(crate) enum Literal<T> {
    //where T: ToHIR {
    LNum(LNumber<T>),
    LStr(LString<T>),
    LRec(LRecord<T>),
    LArr(LArray<T>),
    LCon(LConstructor<T>),
}
macro_rules! make_enum_objects {
    ($($variant:ident = $name:ident($typ:ty)),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name<T>($typ, PhantomData<T>);// where T: ToHIR;
            impl<T> From<$name<T>> for Literal<T> {
                fn from(v: $name<T>) -> Self {
                    Self::$variant(v)
                }
            }
            impl<T> $name<T> {
                pub(crate) fn new(node: $typ) -> Self {
                    Self(node, PhantomData)
                }
                pub(crate) fn text_range(&self) -> ::rowan::TextRange {
                    self.0.text_range()
                }
            }
        )+
    };
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LNumber<T>(SyntaxToken, PhantomData<T>);
impl<T> From<LNumber<T>> for Literal<T> {
    fn from(v: LNumber<T>) -> Self {
        Self::LNum(v)
    }
}
make_enum_objects! {
    LStr = LString(SyntaxNode),
    LRec = LRecord(SyntaxNode),
    LArr = LArray(SyntaxNode),
    LCon = LConstructor(SyntaxNode),
}

impl<T> LNumber<T> {
    pub(crate) fn new(token: SyntaxToken) -> Self {
        Self(token, PhantomData)
    }
    pub(crate) fn text_range(&self) -> ::rowan::TextRange {
        self.0.text_range()
    }
    pub fn value(&self) -> f64 {
        self.0
            .text()
            .to_string()
            .parse()
            .expect("Error parsing number from token")
    }
}
impl<T> ToHIR for LNumber<T>
where
    T: ToHIR,
{
    type HIRType = HigherLiteral<T::HIRType>;
    type ValidationError = ();

    fn to_higher_ast(&self, _line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherLiteral::Number(self.value())
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

pub type StringPart<T> = Either<SmolStr, T>;
impl<T> LString<T>
where
    T: FromSyntaxElement,
{
    pub fn parts(&self) -> Vec<StringPart<T>> {
        let (mut parts, text) = self
            .0
            .children_with_tokens()
            .filter_map(|e| {
                use crate::syntax::{StringToken, SyntaxPart};
                match e.kind() {
                    SyntaxPart::StringToken(StringToken::Text) => e
                        .into_token()
                        .map(|t| StringPart::Left(SmolStr::new(t.text()))),
                    SyntaxPart::StringToken(StringToken::Escape) => e.into_token().map(|t| {
                        StringPart::Left(SmolStr::new(match t.text().chars().last().unwrap() {
                            '$' => "$",
                            '\\' => "\\",
                            'n' => "\n",
                            'r' => "\r",
                            't' => "\t",
                            c => unreachable!("String escape {} should not be possible", c),
                        }))
                    }),
                    SyntaxPart::Context(Context::Expr) => e
                        .into_node()
                        .and_then(|node| T::from_node(Context::Expr, node))
                        .map(|expr| StringPart::Right(expr)),
                    _ => None,
                }
            })
            .fold((Vec::new(), Vec::new()), |(mut acc, mut text), part| {
                match part {
                    StringPart::Left(txt) => {
                        text.push(txt);
                    }
                    expr_segment => {
                        if text.len() > 0 {
                            acc.push(StringPart::Left(SmolStr::new(text.join(""))));
                            text.clear();
                        }
                        acc.push(expr_segment);
                    }
                }
                (acc, text)
            });
        if text.len() > 0 {
            parts.push(StringPart::Left(SmolStr::new(text.join(""))));
        }
        parts
    }
}
impl<T> ToHIR for LString<T>
where
    T: ToHIR + FromSyntaxElement,
{
    type HIRType = HigherLiteral<T::HIRType>;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherLiteral::LStr(
            self.parts()
                .into_iter()
                .map(|part| {
                    higher_ast::expr::literal::StringPart::from(part)//.map_left(|text| text.to_string())
                        .map/*_right*/(|t| t.to_higher_ast(line_lookup))
                })
                .collect(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl<T> LRecord<T>
where
    T: FromSyntaxElement,
{
    pub fn fields(&self) -> Vec<(String, Option<T>)> {
        self.0
            .children()
            .filter_map(|field_node| {
                let mut iter = field_node.children_with_tokens().skip_trivia();
                iter.find(|n| n.kind() == Token::VarName.into())
                    .map(|n| n.into_token().unwrap().text().to_string())
                    .and_then(|name| Some((name.clone(), iter.last().and_then(T::from_element))))
            })
            .collect()
    }
}
impl<T> ToHIR for LRecord<T>
where
    T: ToHIR + FromSyntaxElement,
    T::HIRType: HigherASTWithVar,
{
    type HIRType = HigherLiteral<T::HIRType>;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherLiteral::Record(
            self.fields()
                .into_iter()
                .map(|(name, val)| {
                    let val = val
                        .map(|v| v.to_higher_ast(line_lookup))
                        .unwrap_or(T::HIRType::var_value(name.clone()));
                    (name, val)
                })
                .collect(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl<T> LArray<T>
where
    T: FromSyntaxElement,
{
    pub fn items(&self) -> Vec<T> {
        self.0
            .children()
            .filter_map(|node| {
                if node.kind() == Context::Item.into() {
                    T::from_root_node(node)
                } else {
                    None
                }
            })
            .collect()
    }
}
impl<T> ToHIR for LArray<T>
where
    T: ToHIR + FromSyntaxElement,
{
    type HIRType = HigherLiteral<T::HIRType>;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        HigherLiteral::Array(
            self.items()
                .into_iter()
                .map(|item| item.to_higher_ast(line_lookup))
                .collect(),
        )
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}

impl<T> LConstructor<T>
where
    T: FromSyntaxElement,
{
    pub fn tag(&self) -> Option<SmolStr> {
        self.0
            .find_token(Token::VarName)
            .map(|t| SmolStr::new(t.text()))
    }
    pub fn args(&self) -> Vec<T> {
        self.0
            .find_node(Context::Args)
            .map_or(Vec::default(), |node| {
                node.children_with_tokens()
                    .filter_map(T::from_element)
                    .collect()
            })
    }
}
impl<T> ToHIR for LConstructor<T>
where
    T: ToHIR + FromSyntaxElement,
{
    type HIRType = HigherLiteral<T::HIRType>;
    type ValidationError = ();

    fn to_higher_ast(&self, line_lookup: &line_col::LineColLookup) -> Self::HIRType {
        self.tag()
            .map(|tag| {
                HigherLiteral::Enum(
                    tag.to_string(),
                    self.args()
                        .into_iter()
                        .map(|arg| arg.to_higher_ast(line_lookup))
                        .collect(),
                )
            })
            .unwrap()
    }

    fn validate(&self) -> Option<Self::ValidationError> {
        todo!()
    }
}
