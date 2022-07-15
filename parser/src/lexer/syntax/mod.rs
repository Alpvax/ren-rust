use num_traits::{FromPrimitive, ToPrimitive};

pub(crate) mod context;
pub(crate) mod token;

pub(crate) use context::Context;
pub(crate) use token::{StringToken, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SyntaxPart {
    Error, // = 0
    EOF,

    StringToken(StringToken), // = 2..7
    RawToken(Token),          // 8..255

    Context(context::Context), // 256..
}

impl From<token::Token> for SyntaxPart {
    fn from(t: Token) -> Self {
        if let Token::Error = t {
            Self::Error
        } else {
            Self::RawToken(t)
        }
    }
}
impl From<StringToken> for SyntaxPart {
    fn from(t: StringToken) -> Self {
        if let StringToken::Error = t {
            Self::Error
        } else {
            Self::StringToken(t)
        }
    }
}
impl From<Context> for SyntaxPart {
    fn from(c: Context) -> Self {
        Self::Context(c)
    }
}
impl Default for SyntaxPart {
    fn default() -> Self {
        Self::Context(Context::Module)
    }
}

impl Into<u16> for SyntaxPart {
    fn into(self) -> u16 {
        print!("Converting {:?} to u16", self); //XXX
        let res =//XXX
        match self {
            Self::Error | Self::RawToken(Token::Error) | Self::StringToken(StringToken::Error) => 0,
            Self::EOF => 1,
            Self::StringToken(t) => u16::from(t.to_u8().unwrap()) + 1u16, // StringToken::Error = 0 so no conflict
            Self::RawToken(t) => u16::from(t.to_u8().unwrap()) + 7u16, // Token = 8..255 allowed (actually only currently 57 non-error tokens)
            Self::Context(c) => u16::from(c.to_u8().unwrap()) + 256u16, // Context = 256..
        }
        ;
        println!(": {}", res);
        res //XXX
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SPConvertError {
    StringToken(u16),
    RawToken(u16),
    Context(u16),
}
impl TryFrom<u16> for SyntaxPart {
    type Error = SPConvertError;

    fn try_from(value: u16) -> Result<Self, SPConvertError> {
        Ok(if value == 0 {
            Self::Error
        } else if value == 1 {
            Self::EOF
        } else if value < 7 {
            Self::StringToken(
                StringToken::from_u16(value - 1).ok_or(SPConvertError::StringToken(value))?,
            )
        } else if value <= 0xFF {
            Self::RawToken(Token::from_u16(value - 7).ok_or(SPConvertError::RawToken(value))?)
        } else {
            Self::Context(Context::from_u16(value - 0x100).ok_or(SPConvertError::Context(value))?)
        })
    }
}

#[cfg(test)]
mod test {
    use super::{StringToken, SyntaxPart, Token};

    #[test]
    fn syntaxpart_u16_conversion() {
        for val in 0..512 {
            match SyntaxPart::try_from(val) {
                Ok(
                    sp @ (SyntaxPart::StringToken(StringToken::Error)
                    | SyntaxPart::RawToken(Token::Error)),
                ) => assert_eq!(0u16, sp.into(), "Converting Error: {:?}", sp),
                Ok(sp) => assert_eq!(val, sp.into(), "Converting: {:?}", sp),
                Err(_) => (),
            }
        }
    }
}

pub(super) enum LexerHolder<'source> {
    Main(logos::Lexer<'source, Token>),
    String(logos::Lexer<'source, StringToken>),
    /// Should only be used when morphing in order to take the lexer instance
    None,
}
impl<'source> LexerHolder<'source> {
    pub fn span(&self) -> logos::Span {
        match self {
            LexerHolder::Main(lex) => lex.span(),
            LexerHolder::String(lex) => lex.span(),
            LexerHolder::None => unimplemented!("Should not call methods on LexerType::None"),
        }
    }
    pub fn slice(&self) -> &'source str {
        match self {
            LexerHolder::Main(lex) => lex.slice(),
            LexerHolder::String(lex) => lex.slice(),
            LexerHolder::None => unimplemented!("Should not call methods on LexerType::None"),
        }
    }
    pub(super) fn morph(&mut self) {
        let prev = std::mem::replace(self, Self::None);
        *self = match prev {
            Self::Main(lex) => Self::String(lex.morph()),
            Self::String(lex) => Self::Main(lex.morph()),
            LexerHolder::None => unimplemented!("Should not call methods on LexerType::None"),
        };
    }
}
impl<'source> Iterator for LexerHolder<'source> {
    type Item = (SyntaxPart, &'source str);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LexerHolder::Main(lex) => lex.next().map(|t| (SyntaxPart::RawToken(t), lex.slice())),
            LexerHolder::String(lex) => lex
                .next()
                .map(|t| (SyntaxPart::StringToken(t), lex.slice())),
            LexerHolder::None => unimplemented!("Should not call methods on LexerType::None"),
        }
    }
}
