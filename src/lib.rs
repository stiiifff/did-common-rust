// #![no_main]
// #![no_std]
use std::error::Error;
use std::str::FromStr;
use std::fmt;

extern crate nom;
extern crate json;
mod did_parser;
mod diddoc_parser;

#[derive(Debug, PartialEq)]
pub struct Did<'a> {
    pub method_name: &'a str,
    pub method_specific_id: &'a str,
    pub params: Option<Vec<DidParam<'a>>>,
    pub fragment: Option<&'a str>
}

#[derive(Debug, PartialEq)]
pub struct DidParam<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub enum PublicKeyType {
    Rsa,
    Ed25519,
    EcdsaSecp256k1
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
            _ => Result::Err(ParsePublicKeyTypeError(()))
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
pub enum PublicKeyEncoded {
    Pem(String),
    Jwk(String),
    Hex(String),
    Base64(String),
    Base58(String),
    Multibase(String),
    EthrAddress(String),
    Unsupported
}

const KEYPEM_PROP : &str = "publicKeyPem";
const KEYJWK_PROP : &str = "publicKeyJwk";
const KEYHEX_PROP : &str = "publicKeyHex";
const KEYB58_PROP : &str = "publicKeyBase58";
const KEYB64_PROP : &str = "publicKeyBase64";
const KEYMUL_PROP : &str = "publicKeyMultibase";
const KEYETH_PROP : &str = "ethereumAddress";
const KEY_FORMATS : [&str; 7] = [KEYPEM_PROP, KEYJWK_PROP, KEYHEX_PROP, KEYB58_PROP, KEYB64_PROP, KEYMUL_PROP, KEYETH_PROP];

impl From<(&str,String)> for PublicKeyEncoded {
    fn from(s: (&str,String)) -> Self {
        match s.0 {
            KEYPEM_PROP => PublicKeyEncoded::Pem(s.1),
            KEYJWK_PROP => PublicKeyEncoded::Jwk(s.1),
            KEYHEX_PROP => PublicKeyEncoded::Hex(s.1),
            KEYB58_PROP => PublicKeyEncoded::Base58(s.1),
            KEYB64_PROP => PublicKeyEncoded::Base64(s.1),
            KEYMUL_PROP => PublicKeyEncoded::Multibase(s.1),
            KEYETH_PROP => PublicKeyEncoded::EthrAddress(s.1),
            _ => PublicKeyEncoded::Unsupported
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PublicKey {
    pub id: String,
    pub r#type: PublicKeyType,
    pub controller: String,
    pub pub_key: PublicKeyEncoded
}

#[derive(Debug, PartialEq)]
pub struct DidDocument<'a> {
    pub context: &'a str,
    pub id: String,
    pub pub_keys: Vec<PublicKey>
}

impl<'a> Did<'a> {
    pub fn new(method_name: &'a str, method_specific_id: &'a str) -> Did<'a> {
        Did {
            method_name,
            method_specific_id,
            params: None,
            fragment: None
        }
    }

    pub fn with_params(
        method_name: &'a str,
        method_specific_id: &'a str,
        params: Vec<(&'a str, Option<&'a str>)>,
    ) -> Did<'a> {
        Did {
            method_name,
            method_specific_id,
            params: Some(
                params
                    .iter()
                    .map(|p| DidParam {
                        name: p.0,
                        value: p.1,
                    })
                    .collect(),
            ),
            fragment: None
        }
    }

    pub fn is_valid(did_string: &str) -> bool {
        did_parser::validate_did(did_string)
    } 

    pub fn parse(did_string: &'a str) -> Result<Self, &'a str> {
        match did_parser::parse_did(did_string) {
            Ok((_, did)) => Ok(did),
            Err(_) => Err("Failed to parse did."),
        }
    }
}

impl<'a> DidParam<'a> {
    pub fn new(name: &'a str, value: Option<&'a str>) -> DidParam<'a> {
        DidParam { name, value }
    }
}

impl fmt::Display for DidParam<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}",
            self.name,
            match self.value {
                Some(val) => format!("={}", val),
                None => String::new()
            }
        )
    }
}

impl fmt::Display for Did<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}{}",
            did_parser::DID_SCHEME,
            self.method_name,
            self.method_specific_id,
            match &self.params {
                Some(params) => format!(";{}", params.iter().map(ToString::to_string).collect::<Vec<_>>().join(";")),
                None => String::new()
            }
        )
    }
}

impl<'a> DidDocument<'a> {
    pub fn new<S>(did: S) -> DidDocument<'a> where S: Into<String> {
        DidDocument {
            context: diddoc_parser::GENERIC_DID_CTX,
            id: did.into(),
            pub_keys: vec![]
        }
    }

    pub fn with_pubkeys<S>(did: S, pub_keys: std::vec::Vec<PublicKey>) -> DidDocument<'a> where S: Into<String> {
        DidDocument {
            context: diddoc_parser::GENERIC_DID_CTX,
            id: did.into(),
            pub_keys: pub_keys
        }
    }

    pub fn parse<S>(did_doc: S) -> Result<Self, &'a str> where S: Into<String> {
        diddoc_parser::parse_did_doc(did_doc)
    }
}

#[macro_export]
macro_rules! did {
    ($did: expr) => {
        //TODO: If cannot parse, should generate error w/ parsing error
        Did::parse($did).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::{Did, DidParam};

    #[test]
    fn DidParam_impl_display_trait() { 
        assert_eq!(
            format!("{}", DidParam::new("service", None)),
            "service"            
        );
        assert_eq!(
            format!("{}", DidParam::new("service", Some("agent"))),
            "service=agent"            
        );
    }

    #[test]
    fn did_impl_display_trait() {
        assert_eq!(
            format!("{}", Did::new("example", "")),
            "did:example:"
        );
        assert_eq!(
            format!("{}", Did::new("example", "1234")),
            "did:example:1234"
        );
        assert_eq!(
            format!("{}",
                Did::with_params("example", "1234", vec![
                    ("service", Some("agent"))
                ])
            ),
            "did:example:1234;service=agent"
        );
        assert_eq!(
            format!("{}",
                Did::with_params("example", "1234", vec![
                    ("service", Some("agent")),
                    ("example:foo:bar", Some("baz"))
                ])
            ),
            "did:example:1234;service=agent;example:foo:bar=baz"
        );
    }
}
