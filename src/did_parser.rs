use crate::did::{Did, DidBuilder};

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

pub const DID_SCHEME: &str = "did";
const COLON_SEP: &str = ":";
const SEMICOLON_SEP: &str = ";";
const EQUAL_SEP: &str = "=";
const FRAGMENT_SEP: &str = "#";

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

fn is_fragment_char(c: char) -> bool {
    // 'unreserved' chars
    c.is_ascii_alphanumeric() || //TODO: pct-encoded
    c == '.' || c == '-' || c == '_' || c == '~' ||
    // 'sub-delims' chars
    c == '!' || c == '$' || c == '&' || c == '\'' ||
    c == '(' || c == ')' || c == '*' || c == '+' ||
    c == ',' || c == ';' || c == '=' ||
    // additional 'pchar' chars
    c == ':' || c == '@'
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

fn fragment_char0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        !is_fragment_char(c)
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

fn fragment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Option<&'a str>, E> {
    opt(preceded(tag(FRAGMENT_SEP), fragment_char0))(input)
}

pub fn parse_did<'a>(input: &'a str) -> IResult<&'a str, Did<'a>> {
    let (input, _) = did_scheme(input)?;
    let (input, method_name) = method_name(input)?;
    let (input, method_id) = method_specific_id(input)?;
    let (input, params) = generic_params(input)?;
    let (_empty, fragment) = fragment(input)?;
    assert_eq!(_empty, String::new());

    let mut did = DidBuilder::new(method_name, method_id);
    if let Some(params) = params {
       did = did.with_params(params);
    }
    if let Some(fragment) = fragment {
        did = did.with_fragment(fragment);
    }

    Ok((input, did.build()))
}

pub fn validate_did(input: &str) -> bool {
    did_scheme::<(&str, ErrorKind)>(input)
        .and_then(|(input, _)| method_name(input))
        .and_then(|(input, _)| method_specific_id(input))
        .and_then(|(input, _)| generic_params(input))
        .and_then(|(input, _)| fragment(input))
        .map(|_| true)
        .unwrap_or(false)
}
