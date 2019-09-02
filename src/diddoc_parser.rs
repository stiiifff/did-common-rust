use crate::{
    did::Did,
    did_doc::{DidDocument, PublicKey, PublicKeyEncoded, PublicKeyType, KEY_FORMATS},
};
use json::{parse, JsonValue};
use std::str::FromStr;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";
const CONTEXT_PROP: &str = "@context";
const SUBJECT_PROP: &str = "id";
const PUBKEYS_PROP: &str = "publicKey";

const KEYID_PROP: &str = "id";
const KEYTYPE_PROP: &str = "type";
const KEYCTRL_PROP: &str = "controller";

fn parse_did_context<'a,'b>(json: &'a JsonValue) -> Result<&'b str, &'b str> {
    match json[CONTEXT_PROP].as_str() {
        Some(GENERIC_DID_CTX) => Ok(GENERIC_DID_CTX),
        Some(_) => Err("invalid DID context"),
        None => Err("missing DID context"),
    }
}

fn parse_did_subject<'a,'b>(json: &'a mut JsonValue) -> Result<String, &'b str> {
    match json[SUBJECT_PROP].take_string() {
        Some(sub) => match Did::is_valid(&sub) {
            true => Ok(sub),
            false => Err("invalid DID subject"),
        },
        None => Err("missing DID subject"),
    }
}

fn parse_did_pubkey_list<'a,'b>(json: &'a mut JsonValue) -> Result<Vec<PublicKey>, &'b str> {
    let mut keys: Vec<PublicKey> = vec![];
    for i in 0..json[PUBKEYS_PROP].len() {
        let mut key = json[PUBKEYS_PROP].array_remove(i);
        if key.is_null() {
            break;
        }

        let key_id = parse_did_pubkey_id(&mut key)?;
        if keys.iter().any(|k| k.id == key_id) {
            // return Err(format!("duplicate DID public key id '{}'", key_id).as_str());
            return Err("duplicate DID public key id");
        }

        let key_type = parse_did_pubkey_type(&mut key)?;
        let key_ctrl = parse_did_pubkey_ctrl(&mut key)?;
        let key_format = parse_did_pubkey_format(&mut key)?;
        let key_encoded = parse_did_pubkey_encoded(&mut key, key_format)?;
        
        //TODO: Use PublicKeyBuilder
        keys.push(PublicKey {
            id: key_id,
            r#type: key_type,
            controller: key_ctrl,
            pub_key: key_encoded,
        });
    }
    Ok(keys)
}

fn parse_did_pubkey_id<'a,'b>(key: &'a mut JsonValue) -> Result<String, &'b str> {
    match key[KEYID_PROP].take_string() {
        Some(id) => match Did::is_valid(&id) {
            true => Ok(id),
            false => Err("invalid DID public key id"),
        },
        None => Err("missing DID public key id"),
    }
}

fn parse_did_pubkey_type<'a,'b>(key: &'a mut JsonValue) -> Result<PublicKeyType, &'b str> {
    match key[KEYTYPE_PROP].take_string() {
        Some(r#type) => match PublicKeyType::from_str(&r#type) {
            Ok(key_type) => Ok(key_type),
            Err(_) => Err("invalid DID public key type"),
        },
        None => Err("missing DID public key type"),
    }
}

fn parse_did_pubkey_ctrl<'a,'b>(key: &'a mut JsonValue) -> Result<String, &'b str> {
    match key[KEYCTRL_PROP].take_string() {
        Some(ctrl) => Ok(ctrl),
        None => Err("missing DID public key controller"),
    }
}

fn parse_did_pubkey_format<'a,'b>(key: &'a mut JsonValue) -> Result<&'b str, &'b str> {
    match KEY_FORMATS.iter().find(|f| key.has_key(f)) {
        Some(&kf) => Ok(kf),
        None => Err("missing DID public key property"),
    }
}

fn parse_did_pubkey_encoded<'a,'b>(key: &'a mut JsonValue, key_format: &'a str)
    -> Result<PublicKeyEncoded, &'b str> {
    match key[key_format].take_string() {
        Some(key_enc) => match PublicKeyEncoded::from((key_format, key_enc)) {
            PublicKeyEncoded::Unsupported => Err("unknown DID public key format"),
            supported @ _ => Ok(supported),
        },
        None => Err("missing DID public key controller"),
    }
}

pub fn parse_did_doc<'a, S>(input: S) -> Result<DidDocument<'a>, &'a str>
where
    S: Into<String>,
{
    let mut json = parse(&input.into()).map_err(|_| "Failed to parse did document.")?;

    let _ctx = parse_did_context(&json)?; //TODO: handle additional contexts beyond generic DID context
    let sub = parse_did_subject(&mut json)?;
    let keys = parse_did_pubkey_list(&mut json)?;

    //TODO: Use DidDocumentBuilder
    let mut did_doc = DidDocument::new(sub);
    did_doc.pub_keys = keys;
    Ok(did_doc)
}
