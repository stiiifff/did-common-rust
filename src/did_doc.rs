use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::diddoc_parser;
use json::JsonValue;

#[derive(Debug, PartialEq)]
pub enum PublicKeyType {
    Rsa,
    Ed25519,
    EcdsaSecp256k1,
}

//TODO: implement PublicKeyTypeError see https://doc.rust-lang.org/src/std/net/parser.rs.html#390
//TODO: write tests for PublicKeyType parsing
impl FromStr for PublicKeyType {
    type Err = ParsePublicKeyTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RsaVerificationKey2018" => Ok(Self::Rsa),
            "Ed25519VerificationKey2018" => Ok(Self::Ed25519),
            "Secp256k1VerificationKey2018" => Ok(Self::EcdsaSecp256k1),
            _ => Result::Err(ParsePublicKeyTypeError(())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePublicKeyTypeError(());

impl fmt::Display for ParsePublicKeyTypeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for ParsePublicKeyTypeError {
    fn description(&self) -> &str {
        "invalid DID public key type"
    }
}

//TODO: have a look at the did-common-typescript how they call this thing
// Maybe PublicKey(Formatted)Value ?
#[derive(Debug, PartialEq)]
pub enum PublicKeyEncoded<'a> {
    None,
    Pem(&'a str),
    Jwk(&'a str),
    Hex(&'a str),
    Base64(&'a str),
    Base58(&'a str),
    Multibase(&'a str),
    EthrAddress(&'a str),
    Unsupported,
}

const KEYPEM_PROP: &str = "publicKeyPem";
const KEYJWK_PROP: &str = "publicKeyJwk";
const KEYHEX_PROP: &str = "publicKeyHex";
const KEYB58_PROP: &str = "publicKeyBase58";
const KEYB64_PROP: &str = "publicKeyBase64";
const KEYMUL_PROP: &str = "publicKeyMultibase";
const KEYETH_PROP: &str = "ethereumAddress";
pub const KEY_FORMATS: [&str; 7] = [
    KEYPEM_PROP,
    KEYJWK_PROP,
    KEYHEX_PROP,
    KEYB58_PROP,
    KEYB64_PROP,
    KEYMUL_PROP,
    KEYETH_PROP,
];

impl<'a> From<(&'a str, &'a str)> for PublicKeyEncoded<'a> {
    fn from(s: (&'a str, &'a str)) -> Self {
        match s.0 {
            KEYPEM_PROP => PublicKeyEncoded::Pem(s.1),
            KEYJWK_PROP => PublicKeyEncoded::Jwk(s.1),
            KEYHEX_PROP => PublicKeyEncoded::Hex(s.1),
            KEYB58_PROP => PublicKeyEncoded::Base58(s.1),
            KEYB64_PROP => PublicKeyEncoded::Base64(s.1),
            KEYMUL_PROP => PublicKeyEncoded::Multibase(s.1),
            KEYETH_PROP => PublicKeyEncoded::EthrAddress(s.1),
            _ => PublicKeyEncoded::Unsupported,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PublicKey<'a> {
    id: &'a str,
    key_type: PublicKeyType,
    controller: &'a str,
    encoded_key: PublicKeyEncoded<'a>,
}

impl<'a> PublicKey<'a> {
    pub fn id(&self) -> &'a str {
        self.id
    }

    pub fn key_type(&self) -> &PublicKeyType {
        &self.key_type
    }

    pub fn controller(&self) -> &'a str {
        self.controller
    }

    pub fn encoded_key(&self) -> &PublicKeyEncoded {
        &self.encoded_key
    }
}

#[derive(Debug, PartialEq)]
pub struct PublicKeyBuilder<'a> {
    id: &'a str,
    key_type: PublicKeyType,
    controller: &'a str,
    encoded_key: PublicKeyEncoded<'a>
}

impl<'a> PublicKeyBuilder<'a> {
    pub fn new(id: &'a str, key_type: PublicKeyType, controller: &'a str) -> Self {
        PublicKeyBuilder {
            id,
            key_type,
            controller,
            encoded_key: PublicKeyEncoded::None
        }
    }

    pub fn with_encoded_key(mut self, encoded_key: PublicKeyEncoded<'a>) -> Self {
        self.encoded_key = encoded_key;
        self
    }

    pub fn build(self) -> PublicKey<'a> {
        PublicKey {
            id: self.id,
            key_type: self.key_type,
            controller: self.controller,
            encoded_key: self.encoded_key
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DidDocument<'a> {
    context: &'a str,
    id: &'a str,
    pub_keys: Vec<PublicKey<'a>>,
}

impl<'a> DidDocument<'a> {
    pub fn context(&self) -> &'a str {
        self.context
    }

    pub fn id(&self) -> &'a str {
        self.id
    }

    pub fn pub_keys(&self) -> &[PublicKey<'a>] {
        &self.pub_keys[..]
    }

    pub fn parse(json: &'a JsonValue) -> Result<Self, &'a str> {
        diddoc_parser::parse_did_doc(json)
    }
}

#[derive(Debug, PartialEq)]
pub struct DidDocumentBuilder<'a> {
    context: &'a str,
    id: &'a str,
    pub_keys: Vec<PublicKey<'a>>
}

impl<'a> DidDocumentBuilder<'a> {
    pub fn new(id: &'a str) -> Self
    {
        DidDocumentBuilder {
            context: diddoc_parser::GENERIC_DID_CTX,
            id: id,
            pub_keys: vec![],
        }
    }

    pub fn with_pubkeys(mut self, pub_keys: std::vec::Vec<PublicKey<'a>>) -> Self
    {
        self.pub_keys = pub_keys;
        self
    }

    pub fn build(self) -> DidDocument<'a> {
        DidDocument {
            context : self.context,
            id : self.id,
            pub_keys : self.pub_keys
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DidDocument, DidDocumentBuilder,
        PublicKey, PublicKeyBuilder,
        PublicKeyEncoded, PublicKeyType,
        KEY_FORMATS
    };

    //TODO
}