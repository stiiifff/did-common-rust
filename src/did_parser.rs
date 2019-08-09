use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list;
use nom::AsChar;
use nom::InputTakeAtPosition;
use nom::{
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    sequence::preceded,
    IResult,
};

// Implement a parser for Decentralized Identifiers following the syntax defined at:
// https://w3c-ccg.github.io/did-spec/#generic-did-syntax

const DID_SCHEME: &str = "did";
const COLON_SEP: &str = ":";

#[derive(Debug, PartialEq)]
pub struct DID {
    pub method_name: String,
    pub method_specific_id: String,
}

impl DID {
    pub fn new(method_name: &str, method_specific_id: &str) -> DID {
        DID {
            method_name: method_name.to_string(),
            method_specific_id: method_specific_id.to_string(),
        }
    }

    pub fn parse(did_string: &str) -> Result<Self, &str> {
        match parse_did(did_string) {
            Ok((_, did)) => Ok(did),
            Err(_) => Err("Failed to parse did.")
        }
    }
}

fn did_scheme<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    tag(DID_SCHEME)(input)
}

fn id_char<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !c.is_ascii_alphanumeric() && c != '.' && c != '-' && c != '_'
    })
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

fn method_specific_id<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, String, E> {
    map(
        preceded(tag(COLON_SEP), opt(separated_list(tag(COLON_SEP), id_char))),
        |id: Option<Vec<&'a str>>| match id {
            Some(id_parts) => id_parts.join(COLON_SEP),
            None => String::new(),
        },
    )(input)
}

pub fn parse_did(input: &str) -> IResult<&str, DID> {
    let (input, _) = did_scheme(input)?;
    let (input, method_name) = method_name(input)?;
    let (input, method_id) = method_specific_id(input)?;

    Ok((input, DID::new(method_name, method_id.as_str())))
}
