use crate::{
    did::Did,
    did_doc::{
        DidDocument, DidDocumentBuilder,
        PublicKey, PublicKeyBuilder,
        PublicKeyEncoded, PublicKeyType,
        KEY_FORMATS
    },
};
use json::JsonValue;
use std::str::FromStr;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";
const CONTEXT_PROP: &str = "@context";
const SUBJECT_PROP: &str = "id";
const PUBKEYS_PROP: &str = "publicKey";

const KEYID_PROP: &str = "id";
const KEYTYPE_PROP: &str = "type";
const KEYCTRL_PROP: &str = "controller";

fn parse_did_context<'a>(json: &'a JsonValue) -> Result<&'a str, &'a str> {
    match json[CONTEXT_PROP].as_str() {
        Some(GENERIC_DID_CTX) => Ok(GENERIC_DID_CTX),
        Some(_) => Err("invalid DID context"),
        None => Err("missing DID context"),
    }
}

fn parse_did_subject<'a>(json: &'a JsonValue) -> Result<&'a str, &'a str> {
    match json[SUBJECT_PROP].as_str() {
        Some(sub) => match Did::is_valid(sub) {
            true => Ok(sub),
            false => Err("invalid DID subject"),
        },
        None => Err("missing DID subject"),
    }
}

fn parse_did_pubkey_list<'a>(json: &'a JsonValue) -> Result<Vec<PublicKey>, &'a str> {
    let mut keys: Vec<PublicKey> = vec![];
    for i in 0..json[PUBKEYS_PROP].len() {
        let key = &json[PUBKEYS_PROP][i];
        if key.is_null() {
            break;
        }

        let key_id = parse_did_pubkey_id(key)?;
        if keys.iter().any(|k| k.id() == key_id) {
            // return Err(format!("duplicate DID public key id '{}'", key_id).as_str());
            return Err("duplicate DID public key id");
        }

        let key_type = parse_did_pubkey_type(key)?;
        let key_ctrl = parse_did_pubkey_ctrl(key)?;
        let key_format = parse_did_pubkey_format(key)?;
        let key_encoded = parse_did_pubkey_encoded(key, key_format)?;
        
        keys.push(
            PublicKeyBuilder::new(key_id, key_type, key_ctrl)
                .with_encoded_key(key_encoded)
                .build()
        );
    }
    Ok(keys)
}

fn parse_did_pubkey_id<'a>(key: &'a JsonValue) -> Result<&'a str, &'a str> {
    match key[KEYID_PROP].as_str() {
        Some(id) => match Did::is_valid(&id) {
            true => Ok(id),
            false => Err("invalid DID public key id"),
        },
        None => Err("missing DID public key id"),
    }
}

fn parse_did_pubkey_type<'a>(key: &'a JsonValue) -> Result<PublicKeyType, &'a str> {
    match key[KEYTYPE_PROP].as_str() {
        Some(r#type) => match PublicKeyType::from_str(&r#type) {
            Ok(key_type) => Ok(key_type),
            Err(_) => Err("invalid DID public key type"),
        },
        None => Err("missing DID public key type"),
    }
}

fn parse_did_pubkey_ctrl<'a>(key: &'a JsonValue) -> Result<&'a str, &'a str> {
    match key[KEYCTRL_PROP].as_str() {
        Some(ctrl) => Ok(ctrl),
        None => Err("missing DID public key controller"),
    }
}

fn parse_did_pubkey_format<'a>(key: &'a JsonValue) -> Result<&'a str, &'a str> {
    match KEY_FORMATS.iter().find(|f| key.has_key(f)) {
        Some(&kf) => Ok(kf),
        None => Err("missing DID public key property"),
    }
}

fn parse_did_pubkey_encoded<'a>(key: &'a JsonValue, key_format: &'a str) -> Result<PublicKeyEncoded<'a>, &'a str> {
    match key[key_format].as_str() {
        Some(key_enc) => match PublicKeyEncoded::from((key_format, key_enc)) {
            PublicKeyEncoded::Unsupported => Err("unknown DID public key format"),
            supported @ _ => Ok(supported),
        },
        None => Err("missing DID public key controller"),
    }
}

pub fn parse_did_doc<'a>(json: &'a JsonValue) -> Result<DidDocument<'a>, &'a str>
{
    let _ctx = parse_did_context(json)?; //TODO: handle additional contexts beyond generic DID context
    let sub = parse_did_subject(json)?;
    let keys = parse_did_pubkey_list(json)?;

    let did_doc = DidDocumentBuilder::new(sub)
        .with_pubkeys(keys)
        .build();
    Ok(did_doc)
}
