use nom::bytes::complete::tag;
use nom::character::complete::{char as nom_char, space0, space1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

use crate::names::*;
use crate::string_parser::parse_string;
#[derive(Debug, PartialEq)]
pub struct Import {
    path: String,
    namespace: Option<Namespace>,
    exposing: Option<Vec<VarName>>,
}
impl Import {
    pub fn new(path: &str, namespace: Option<Vec<&str>>, exposing: Option<Vec<&str>>) -> Self {
        Import {
            path: path.to_owned(),
            namespace: namespace.map(|v| Namespace(v.iter().map(|&s| s.to_owned()).collect())),
            exposing: exposing.map(|v| v.iter().map(|&s| VarName(s.to_owned())).collect()),
        }
    }
    pub fn of(path: String, namespace: Option<Namespace>, exposing: Option<Vec<VarName>>) -> Self {
        Import {
            path,
            namespace,
            exposing,
        }
    }
}

fn parse_as(input: &str) -> IResult<&str, Namespace> {
    preceded(tuple((space1, tag("as"), space1)), parse_namespaces)(input)
}

fn parse_exposing(input: &str) -> IResult<&str, Vec<VarName>> {
    preceded(
        tuple((space1, tag("exposing"), space0)),
        delimited(
            tuple((nom_char('{'), space0)),
            separated_list1(tuple((space0, nom_char(','), space0)), parse_name),
            tuple((space0, nom_char('}'))),
        ),
    )(input)
}

pub fn parse_import(input: &str) -> IResult<&str, Import> {
    let (input, (_, _, path)) = tuple((tag("import"), space1, parse_string))(input)?;

    let (input, namespace) = opt(parse_as)(input)?;
    /*println!(
        "path = {}; namespace = {:?}\n input=\"{}\"",
        path, namespace, input
    );*/

    let (input, exposing) = opt(parse_exposing)(input)?;
    //println!("exposing = {:?}", exposing);

    Ok((
        input,
        Import {
            path: path.to_string(),
            namespace,
            exposing,
        },
    ))
}
