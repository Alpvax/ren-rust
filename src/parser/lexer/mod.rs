pub mod token;

mod test;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::convert::TryInto;

use logos::{Logos, Span, SpannedIter};
pub use token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub token: Token,
    pub span: Span,
}
impl Lexeme {
    pub fn convert_to<'a, T: From<&'a Token>>(&'a self) -> T {
        T::from(&self.token)
    }
}
impl From<(Token, Span)> for Lexeme {
    fn from((token, span): (Token, Span)) -> Self {
        Lexeme { token, span }
    }
}
impl From<Lexeme> for (Token, Span) {
    fn from(l: Lexeme) -> Self {
        (l.token, l.span)
    }
}
impl From<Lexeme> for Token {
    fn from(l: Lexeme) -> Self {
        l.token
    }
}
impl From<Lexeme> for Span {
    fn from(l: Lexeme) -> Self {
        l.span
    }
}
impl From<&Lexeme> for Token {
    fn from(l: &Lexeme) -> Self {
        l.token.clone()
    }
}

pub struct LexemeGroup<const N: usize> {
    size: usize,
    values: Vec<Lexeme>,
}
#[allow(dead_code)]
impl<const N: usize> LexemeGroup<N> {
    pub fn as_token_array(&self) -> [Option<&Token>; N] {
        let mut res = [None; N];
        for (i, l) in self.values.iter().enumerate() {
            if i >= N {
                break;
            }
            res[i] = Some(&l.token);
        }
        res
    }
    pub fn as_token_array_unchecked(&self) -> [&Token; N] {
        self.as_token_array()
            .iter()
            .map(|&o| o.unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
    pub fn len_total(&self) -> usize {
        self.size
    }
    pub fn len_valid(&self) -> usize {
        self.values.len()
    }
    pub fn is_complete(&self) -> bool {
        self.len_total() == self.len_valid()
    }
    pub fn get(&self, index: usize) -> Option<&Lexeme> {
        self.values.get(index)
    }
    pub fn subgroup<const START: usize, const LEN: usize>(&self) -> LexemeGroup<LEN> {
        LexemeGroup::<LEN> {
            size: LEN,
            values: (&self.values[START..START + LEN]).into(),
        }
    }
}
impl<const N: usize> From<[Lexeme; N]> for LexemeGroup<N> {
    fn from(arr: [Lexeme; N]) -> Self {
        Self {
            size: N,
            values: arr.into(),
        }
    }
}
impl<const N: usize> From<&[Lexeme]> for LexemeGroup<N> {
    fn from(arr: &[Lexeme]) -> Self {
        Self {
            size: N,
            values: arr.into(),
        }
    }
}
/// Assumes [Some... None] (i.e. no None after Some entries, otherwise order is lost)
impl<const N: usize> From<[Option<Lexeme>; N]> for LexemeGroup<N> {
    fn from(arr: [Option<Lexeme>; N]) -> Self {
        Self {
            size: N,
            values: arr.iter().filter_map(|o| o.clone()).collect(),
        }
    }
}
/// Assumes [Some... None] (i.e. no None after Some entries, otherwise order is lost)
impl<const N: usize> From<[Option<&Lexeme>; N]> for LexemeGroup<N> {
    fn from(arr: [Option<&Lexeme>; N]) -> Self {
        Self {
            size: N,
            values: arr.iter().filter_map(|&o| o.map(|l| l.clone())).collect(),
        }
    }
}
/// Assumes [Some... None] (i.e. no None after Some entries, otherwise order is lost)
impl<const N: usize> From<&[Option<Lexeme>]> for LexemeGroup<N> {
    fn from(arr: &[Option<Lexeme>]) -> Self {
        Self {
            size: N,
            values: arr.iter().filter_map(|o| o.clone()).collect(),
        }
    }
}

type LexerInternal<'s> = std::iter::Map<SpannedIter<'s, Token>, fn((Token, Span)) -> Lexeme>;

pub struct Lexer<'s> {
    lex: RefCell<LexerInternal<'s>>,
    peekable: VecDeque<Lexeme>,
}
#[allow(dead_code)]
impl<'s> Lexer<'s> {
    pub fn new(input: &'s str) -> Self {
        Self {
            lex: RefCell::new(Token::lexer(input).spanned().map(Lexeme::from)),
            peekable: VecDeque::with_capacity(1),
        }
    }
    fn _next_internal(&self) -> Option<Lexeme> {
        self.lex.borrow_mut().next()
    }
    fn _peek_next(&mut self) -> Option<&Lexeme> {
        if let Some(nxt) = self._next_internal() {
            self.peekable.push_back(nxt);
            self.peekable.back()
        } else {
            None
        }
    }
    fn _peek_to_n(&mut self, n: usize) -> bool {
        for _ in self.peekable.len()..n {
            if let None = self._peek_next() {
                return false;
            }
        }
        true
    }
    pub fn peek(&mut self) -> Option<&Lexeme> {
        if self.peekable.len() < 1 {
            self._peek_next()
        } else {
            self.peekable.front()
        }
    }
    pub fn peek_offset(&mut self, offset: usize) -> Option<&Lexeme> {
        if self._peek_to_n(offset + 1) {
            self.peekable.get(offset)
        } else {
            None
        }
    }
    pub fn peek_n<const N: usize>(&mut self) -> LexemeGroup<N> {
        self._peek_to_n(N);
        let mut res = [None; N];
        for (i, l) in self.peekable.iter().enumerate() {
            if i >= N {
                break;
            }
            res[i] = Some(l);
        }
        res.into()
    }
    pub fn peek_n_exact<const N: usize>(&mut self) -> Option<LexemeGroup<N>> {
        self._peek_to_n(N);
        if self.peekable.len() < N {
            None
        } else {
            Some(self.peekable.as_slices().0[..N].into()) //try_into().unwrap())
        }
    }
    pub fn next_token(&mut self) -> Option<Token> {
        self.next().map(|l| l.token)
    }
    pub fn peek_token(&mut self) -> Option<&Token> {
        self.peek().map(|l| &l.token)
    }
    pub fn into_tokens_iter(self) -> impl Iterator<Item = Token> + 's {
        self.map(|l| l.token)
    }
    pub fn remaining_tokens_vec(self) -> Vec<Token> {
        self.into_tokens_iter().collect()
    }
}
impl Iterator for Lexer<'_> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        self.peekable.pop_front().or_else(|| self._next_internal())
    }
}
