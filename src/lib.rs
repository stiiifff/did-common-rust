// #![no_main]
// #![no_std]
use std::fmt;

extern crate nom;
mod did_parser;

#[derive(Debug, PartialEq)]
pub struct DID<'a> {
    pub method_name: &'a str,
    pub method_specific_id: &'a str,
    pub params: Option<Vec<DIDParam<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct DIDParam<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

impl<'a> DID<'a> {
    pub fn new(method_name: &'a str, method_specific_id: &'a str) -> DID<'a> {
        DID {
            method_name,
            method_specific_id,
            params: None,
        }
    }

    pub fn with_params(
        method_name: &'a str,
        method_specific_id: &'a str,
        params: Vec<(&'a str, Option<&'a str>)>,
    ) -> DID<'a> {
        DID {
            method_name,
            method_specific_id,
            params: Some(
                params
                    .iter()
                    .map(|p| DIDParam {
                        name: p.0,
                        value: p.1,
                    })
                    .collect(),
            ),
        }
    }

    pub fn parse(did_string: &'a str) -> Result<Self, &'a str> {
        match did_parser::parse_did(did_string) {
            Ok((_, did)) => Ok(did),
            Err(_) => Err("Failed to parse did."),
        }
    }
}

impl<'a> DIDParam<'a> {
    pub fn new(name: &'a str, value: Option<&'a str>) -> DIDParam<'a> {
        DIDParam { name, value }
    }
}

impl fmt::Display for DIDParam<'_> {
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

impl fmt::Display for DID<'_> {
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

#[macro_export]
macro_rules! did {
    ($did: expr) => {
        //TODO: If cannot parse, should generate error w/ parsing error
        DID::parse($did).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::{DID, DIDParam};

    #[test]
    fn didparam_impl_display_trait() { 
        assert_eq!(
            format!("{}", DIDParam::new("service", None)),
            "service"            
        );
        assert_eq!(
            format!("{}", DIDParam::new("service", Some("agent"))),
            "service=agent"            
        );
    }

    #[test]
    fn did_impl_display_trait() {
        assert_eq!(
            format!("{}", DID::new("example", "")),
            "did:example:"
        );
        assert_eq!(
            format!("{}", DID::new("example", "1234")),
            "did:example:1234"
        );
        assert_eq!(
            format!("{}",
                DID::with_params("example", "1234", vec![
                    ("service", Some("agent"))
                ])
            ),
            "did:example:1234;service=agent"
        );
        assert_eq!(
            format!("{}",
                DID::with_params("example", "1234", vec![
                    ("service", Some("agent")),
                    ("example:foo:bar", Some("baz"))
                ])
            ),
            "did:example:1234;service=agent;example:foo:bar=baz"
        );
    }
}
