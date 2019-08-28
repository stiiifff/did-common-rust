use std::collections::HashMap;
use super::{DID, DIDDocument, PublicKey};
use json::parse;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";
const CONTEXT_PROP: &str = "@context";
const SUBJECT_PROP: &str = "id";
const PUBKEYS_PROP: &str = "publicKey";
const KEYID_PROP  : &str = "id";
const KEYTYPE_PROP: &str = "type";
const KEYCTRL_PROP: &str = "controller";

pub fn parse_did_doc<'a, S>(input: S) -> Result<DIDDocument<'a>, &'a str> where S: Into<String> {
    let mut json = parse(&input.into()).map_err(|_| "Failed to parse did document.")?;

    let _ctx = match json[CONTEXT_PROP].as_str() {
        Some(GENERIC_DID_CTX) => Ok(GENERIC_DID_CTX),
        Some(_) => Err("invalid DID context"),
        None => Err("missing DID context")
    }?; //TODO: handle additional contexts beyond generic DID context

    let did_doc = match json[SUBJECT_PROP].take_string() {
        Some(sub) => {
            match DID::is_valid(&sub) {
                true => Ok(DIDDocument::new(sub)),
                false => Err("invalid DID subject")
            }
        },
        None => Err("missing DID subject")
    }?;
    
    let mut keys = HashMap::new();
    for i in 0..json[PUBKEYS_PROP].len() {
        let mut key = json[PUBKEYS_PROP].array_remove(i);
        if key.is_null() {
            break;
        }
        
        let key_id = match key[KEYID_PROP].take_string() {
            Some(id) => {
                match DID::is_valid(&id) {
                    true => Ok(id),
                    false => Err("invalid DID public key id")
                }
            },
            None => Err("missing DID public key id")
        }?;

        if keys.contains_key(&key_id) {
            return Err(format!("duplicate DID public key id '{}'", key_id).as_str());
        }

        let key_type = match key[KEYTYPE_PROP].take_string() {
            Some(r#type) => Ok(r#type),
            None => Err("missing DID public key type")
        }?;

        let key_ctrl = match key[KEYCTRL_PROP].take_string() {
            Some(ctrl) => Ok(ctrl),
            None => Err("missing DID public key controller")
        }?;

        keys.insert(&key_id, PublicKey {
            id: &key_id,
            r#type: &key_type,
            controller: &key_ctrl,
            pub_key: PublicKeyFormat::Pem("")
        });
    } 

    //TODO: key IDs must be unique

    Ok(did_doc)
}
