use crate::DID;

use nom::{
    bytes::complete::tag,
    combinator::{opt, recognize},
    error::{ErrorKind, ParseError},
    multi::separated_list,
    sequence::{preceded, separated_pair},
    AsChar, IResult, InputTakeAtPosition,
};

// Implement a parser for Decentralized Identifiers following the syntax defined at:
// https://w3c-ccg.github.io/did-spec/#generic-did-syntax

const DID_SCHEME: &str = "did";
const COLON_SEP: &str = ":";
const SEMICOLON_SEP: &str = ";";
const EQUAL_SEP: &str = "=";

fn did_scheme<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    tag(DID_SCHEME)(input)
}

fn is_method_char(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_digit()
}

fn method_char<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !is_method_char(c)
        },
        ErrorKind::AlphaNumeric,
    )
}

fn is_param_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ':' //TODO: pct-encoded
}

fn param_char1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !is_param_char(c)
        },
        ErrorKind::AlphaNumeric,
    )
}

fn param_char0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !is_param_char(c)
    })
}

fn method_name<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(tag(COLON_SEP), method_char)(input)
}

fn is_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_'
}

fn id_char<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            !is_id_char(c)
        },
        ErrorKind::AlphaNumeric,
    )
}

fn method_specific_id<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(
        tag(COLON_SEP),
        recognize(opt(separated_list(tag(COLON_SEP), id_char))),
    )(input)
}

fn generic_params<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Option<Vec<(&'a str, Option<&'a str>)>>, E> {
    opt(preceded(
        tag(SEMICOLON_SEP),
        separated_list(
            tag(SEMICOLON_SEP),
            separated_pair(param_char1, tag(EQUAL_SEP), opt(param_char0)),
        ),
    ))(input)
}

pub fn parse_did<'a>(input: &'a str) -> IResult<&'a str, DID<'a>> {
    let (input, _) = did_scheme(input)?;
    let (input, method_name) = method_name(input)?;
    let (input, method_id) = method_specific_id(input)?;
    let (input, params) = generic_params(input)?;

    let did = match params {
        Some(params) => DID::with_params(method_name, method_id, params),
        None => DID::new(method_name, method_id),
    };

    Ok((input, did))
}
