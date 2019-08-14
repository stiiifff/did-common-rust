use nom::AsChar;
use nom::InputTakeAtPosition;
use nom::{
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    sequence::preceded,
    IResult,
};
use regex::Regex;

// Implement a parser for Decentralized Identifiers following the syntax defined at:
// https://w3c-ccg.github.io/did-spec/#generic-did-syntax

const DID_SCHEME: &str = "did";
const COLON_SEP: &str = ":";

#[derive(Debug, PartialEq)]
pub struct DID<'a> {
    pub method_name: &'a str,
    pub method_specific_id: &'a str,
}

impl<'a> DID<'a> {
    pub fn new(method_name: &'a str, method_specific_id: &'a str) -> DID<'a> {
        DID {
            method_name: method_name,
            method_specific_id: method_specific_id,
        }
    }

    pub fn parse(did_string: &'a str) -> Result<Self, &'a str> {
        match parse_did(did_string) {
            Ok((_, did)) => Ok(did),
            Err(_) => Err("Failed to parse did.")
        }
    }
}

fn did_scheme<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    tag(DID_SCHEME)(input)
}

fn method_char<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !c.is_ascii_lowercase() && !c.is_ascii_digit()
        },
        ErrorKind::AlphaNumeric,
    )
}

fn method_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(tag(COLON_SEP), method_char)(input)
}

fn method_specific_id<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    lazy_static! {
        static ref ID_CHAR_REGEX: Regex = Regex::new("^[[:alnum:]._-]*(:[[:alnum:]._-]*)*").unwrap();
    }
    let (input, _) = tag(COLON_SEP)(input)?;

    match ID_CHAR_REGEX.find(input) {
        Some(mat) => Ok((&input[mat.end()..], mat.as_str())),
        None => Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::RegexpFind)))
    }
}

pub fn parse_did<'a>(input: &'a str) -> IResult<&'a str, DID<'a>> {
    let (input, _) = did_scheme(input)?;
    let (input, method_name) = method_name(input)?;
    let (input, method_id) = method_specific_id(input)?;

    Ok((input, DID::new(method_name, method_id)))
}
