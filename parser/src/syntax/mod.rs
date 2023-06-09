use num_traits::{FromPrimitive, ToPrimitive};

mod context;
pub(crate) mod lexer;

pub(crate) use context::Context;
pub(crate) use lexer::{StringToken, Token, TokenType};
use rowan::{Language, SyntaxKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::From)]
pub enum SyntaxPart {
    Error, // = 0
    EOF,   // = 1

    StringToken(StringToken), // = 2..7
    Token(Token),             // 8..255

    Context(context::Context), // 256..
}

impl Default for SyntaxPart {
    fn default() -> Self {
        Self::Context(Context::Module)
    }
}

impl Into<u16> for SyntaxPart {
    fn into(self) -> u16 {
        match self {
            Self::Error => 0,
            Self::EOF => 1,
            Self::StringToken(t) => u16::from(t.to_u8().unwrap()) + 2u16, // StringToken = 2..=7 allowed (actually only currently 4 tokens)
            Self::Token(t) => u16::from(t.to_u8().unwrap()) + 8u16, // Token = 8..=255 allowed (actually only currently 60 tokens)
            Self::Context(c) => u16::from(c.to_u8().unwrap()) + 256u16, // Context = 256..
        }
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
            Self::Token(Token::from_u16(value - 7).ok_or(SPConvertError::RawToken(value))?)
        } else {
            Self::Context(Context::from_u16(value - 0x100).ok_or(SPConvertError::Context(value))?)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenLang;
impl Language for RenLang {
    type Kind = SyntaxPart;

    fn kind_from_raw(raw: SyntaxKind) -> Self::Kind {
        Self::Kind::try_from(raw.0).expect("Failed converting rowan::SyntaxKind to SyntaxPart!")
    }

    fn kind_to_raw(kind: Self::Kind) -> SyntaxKind {
        SyntaxKind(kind.into())
    }
}

pub(crate) type SyntaxNode = rowan::SyntaxNode<RenLang>;

#[cfg(test)]
mod test {
    use super::SyntaxPart;

    #[test]
    fn syntaxpart_u16_conversion() {
        for val in 0..512 {
            match SyntaxPart::try_from(val) {
                Ok(sp) => assert_eq!(
                    val,
                    <SyntaxPart as Into<u16>>::into(sp),
                    "Converting: {:?}",
                    sp
                ),
                Err(_) => (),
            }
        }
    }
}

impl From<TokenType> for SyntaxPart {
    fn from(t: TokenType) -> Self {
        match t {
            TokenType::Token(tok) => Self::Token(tok),
            TokenType::String(tok) => Self::StringToken(tok),
            TokenType::None => Self::EOF,
        }
    }
}
impl TryFrom<SyntaxPart> for TokenType {
    type Error = SyntaxPart;

    fn try_from(value: SyntaxPart) -> Result<Self, Self::Error> {
        match value {
            SyntaxPart::Token(tok) => Ok(Self::Token(tok)),
            SyntaxPart::StringToken(tok) => Ok(Self::String(tok)),
            val => Err(val),
        }
    }
}
