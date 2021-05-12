use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use lazy_static::lazy_static;
use nom::character::complete::char as nom_char;
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::regex::Regex;
use nom::sequence::preceded;
use nom::IResult;

lazy_static! {
    static ref NS_PATTERN_START: Regex = Regex::new(r"^([A-Z][a-zA-Z0-9]*)\b").unwrap();
    static ref VAR_PATTERN_START: Regex = Regex::new(r"^([a-z][a-zA-Z0-9]*)\b").unwrap();
    static ref FULL_PATTERN_START: Regex = Regex::new(
        r"((?:[A-Z][A-Za-z0-9]*)(?:\.[A-Z][A-Za-z0-9]*)*)(?:\.([a-z][A-Za-z0-9]*))?|([a-z][A-Za-z0-9]*)\b"
    ).unwrap();

    /*static ref NS_PATTERN_SINGLE: Regex = Regex::new(r"\b([A-Z][a-zA-Z0-9]*)\b").unwrap();
    static ref NS_PATTERN_MULT: Regex = Regex::new(r"\b([A-Z][a-zA-Z0-9]*)(\.[A-Z][a-zA-Z0-9]*)*\b").unwrap();
    static ref VAR_PATTERN: Regex = Regex::new(r"\b([a-z][a-zA-Z0-9]*)\b").unwrap();*/
    //static ref NS_VAR_PATTERN: Regex = Regex::new(r"\b([A-Z][a-zA-Z0-9]*)(\.[A-Z][a-zA-Z0-9]*)*\.([a-z][a-zA-Z0-9]*)\b").unwrap();
}
#[derive(Debug)]
pub enum Identifier {
    VarName(VarName),
    Namespace(Namespace),
    Namespaced(Namespace, VarName),
}
impl FromStr for Identifier {
    type Err = (); //TODO: Better error

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match FULL_PATTERN_START.captures(s) {
            Some(caps) => {
                match (
                    caps.get(1)
                        .map(|m| Namespace(m.as_str().split('.').map(|s| s.to_owned()).collect())),
                    caps.get(2)
                        .or(caps.get(3))
                        .map(|m| VarName(m.as_str().to_owned())),
                ) {
                    (Some(ns), Some(v)) => Ok(Self::Namespaced(ns, v)),
                    (Some(ns), None) => Ok(Self::Namespace(ns)),
                    (None, Some(v)) => Ok(Self::VarName(v)),
                    _ => Err(()),
                }
            }
            None => Err(()),
        }
    }
}
impl From<VarName> for Identifier {
    fn from(n: VarName) -> Self {
        Self::VarName(n)
    }
}
impl From<Namespace> for Identifier {
    fn from(n: Namespace) -> Self {
        Self::Namespace(n)
    }
}
impl From<(Namespace, VarName)> for Identifier {
    fn from((ns, v): (Namespace, VarName)) -> Self {
        Self::Namespaced(ns, v)
    }
}
impl TryFrom<(Option<Namespace>, Option<VarName>)> for Identifier {
    type Error = String;

    fn try_from(value: (Option<Namespace>, Option<VarName>)) -> Result<Self, Self::Error> {
        match value {
            (Some(ns), Some(v)) => Ok(Self::Namespaced(ns, v)),
            (Some(ns), None) => Ok(Self::Namespace(ns)),
            (None, Some(v)) => Ok(Self::VarName(v)),
            _ => Err("An Identifier must have either a namespace or a name (or both)".to_owned()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarName(pub String);
/*impl FromStr for VarName {
    type Err = (); //TODO: Better error

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = VAR_PATTERN_START.captures(s) {
            Ok(Self(caps.get(1).unwrap().as_str().to_owned()))
        } else {
            Err(())
        }
    }
}*/

#[derive(Debug, PartialEq, Eq)]
pub struct Namespace(pub Vec<String>);
/*impl FromStr for Namespace {
    type Err = (); //TODO: Better error

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = NS_PATTERN_START.captures(s) {
            Ok(Self(caps.get(1).unwrap().as_str().to_owned()))
        } else {
            Err(())
        }
    }
}*/

fn regex_error(input: &str) -> nom::Err<nom::error::Error<&str>> {
    nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::RegexpMatch,
    ))
}

pub fn parse_name(input: &str) -> IResult<&str, VarName> {
    match VAR_PATTERN_START.find(input) {
        Some(m) => Ok((&input[m.end()..], VarName(m.as_str().to_owned()))),
        None => Err(regex_error(input)),
    }
}
pub fn parse_single_namespace(input: &str) -> IResult<&str, String> {
    match NS_PATTERN_START.find(input) {
        Some(m) => Ok((&input[m.end()..], m.as_str().to_owned())),
        None => Err(regex_error(input)),
    }
}
pub fn parse_namespaces(input: &str) -> IResult<&str, Namespace> {
    map(
        separated_list1(nom_char('.'), parse_single_namespace),
        Namespace,
    )(input)
}
pub fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    let (input, ns) = opt(parse_namespaces)(input)?;
    let (input, vn) = if ns.is_some() {
        opt(preceded(nom_char('.'), parse_name))(input)?
    } else {
        opt(parse_name)(input)?
    };
    match TryInto::<Identifier>::try_into((ns, vn)) {
        Ok(i) => Ok((input, i)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::IsNot,
        ))),
    }
}
