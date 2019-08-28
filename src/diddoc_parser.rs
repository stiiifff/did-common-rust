use super::{DID, DIDDocument};
use json::parse;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";
const CONTEXT_PROP: &str = "@context";
const SUBJECT_PROP: &str = "id";

pub fn parse_did_doc<'a, S>(input: S) -> Result<DIDDocument<'a>, &'a str> where S: Into<String> {
    let mut json = parse(&input.into()).map_err(|_| "Failed to parse did document.")?;

    let _ctx = match json[CONTEXT_PROP].as_str() {
        Some(GENERIC_DID_CTX) => Ok(GENERIC_DID_CTX),
        Some(_) => Err("invalid DID context"),
        None => Err("missing DID context")
    }?; //TODO: handle additional contexts beyond generic DID context

    match json[SUBJECT_PROP].take_string() {
        Some(sub) => {
            match DID::is_valid(&sub) {
                true => Ok(DIDDocument::new(sub)),
                false => Err("invalid DID subject")
            }
        },
        None => Err("missing DID subject")
    }
}
