//! Based on the nom example of how to parse an escaped string from
//! https://github.com/Geal/nom/blob/8e09f0c3029d32421b5b69fb798cef6855d0c8df/examples/string.rs. The
//! rules for the string are as follows:
//!
//! - Enclosed by either single or double quotes
//! - Can contain any raw unescaped code point besides \ and (' or ")
//! - Matches the following escape sequences: \n, \r, \t, (\' or \"), \\

use std::fmt::Debug;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::streaming::is_not;
use nom::character::streaming::char as nom_char;
use nom::combinator::{map, value, verify};
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedString {
    DoubleQuoted(String),
    SingleQuoted(String),
}
impl ParsedString {
    pub fn get_str<'s>(&'s self) -> &'s str {
        match self {
            ParsedString::DoubleQuoted(s) => s,
            ParsedString::SingleQuoted(s) => s,
        }
    }
    pub fn unwrap(self) -> String {
        match self {
            ParsedString::DoubleQuoted(s) => s,
            ParsedString::SingleQuoted(s) => s,
        }
    }
}
impl std::fmt::Display for ParsedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.get_str(), f)
    }
}
impl From<ParsedString> for String {
    fn from(s: ParsedString) -> Self {
        s.unwrap()
    }
}
impl FromStr for ParsedString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::DoubleQuoted(s.to_owned()))
    }
}

/// Parse an escaped character: \n, \t, \r, \u{00AC}, etc.
fn parse_escaped_char<'s>(input: &'s str, terminator: char) -> IResult<&'s str, char> {
    preceded(
        nom_char('\\'),
        // `alt` tries each parser in sequence, returning the result of
        // the first successful match
        alt((
            // The `value` parser returns a fixed value (the first argument) if its
            // parser (the second argument) succeeds. In these cases, it looks for
            // the marker characters (n, r, t, etc) and returns the matching
            // character (\n, \r, \t, etc).
            value('\n', nom_char('n')),
            value('\r', nom_char('r')),
            value('\t', nom_char('t')),
            value('\\', nom_char('\\')),
            //value('/', char('/')),
            value(terminator, nom_char(terminator)),
        )),
    )(input)
}

/// Parse a non-empty block of text that doesn't include \ or "
fn parse_literal<'s>(input: &'s str, nonliterals: &[u8]) -> IResult<&'s str, &'s str> {
    // `verify` runs a parser, then runs a verification function on the output of
    // the parser. The verification function accepts out output only if it
    // returns true. In this case, we want to ensure that the output of is_not
    // is non-empty.
    verify(is_not(nonliterals), |s: &str| !s.is_empty())(input)
}

enum StringFragment<'s> {
    Literal(&'s str),
    Escape(char),
}

fn parse_terminated<'s, 't>(terminator: u8) -> impl FnMut(&'s str) -> IResult<&'s str, String> {
    let parse_lit = move |input: &'s str| parse_literal(input, &[b'\\', terminator]);
    let c = terminator.into();
    let parse_escape = move |input: &'s str| parse_escaped_char(input, c);
    delimited(
        nom_char(c),
        fold_many0(
            alt((
                map(parse_lit, StringFragment::Literal),
                map(parse_escape, StringFragment::Escape),
            )),
            String::new(),
            |mut string, fragment| {
                match fragment {
                    StringFragment::Literal(s) => string.push_str(s),
                    StringFragment::Escape(c) => string.push(c),
                }
                string
            },
        ),
        nom_char(c),
    )
}

/// Parse a string. Use a loop of parse_fragment and push all of the fragments
/// into an output string.
pub fn parse_string(input: &str) -> IResult<&str, ParsedString> {
    alt((
        map(parse_terminated(b'"'), ParsedString::DoubleQuoted),
        map(parse_terminated(b'\''), ParsedString::SingleQuoted),
    ))(input)
}
