// #![no_main]
// #![no_std]

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

#[macro_export]
macro_rules! did {
    ($did: expr) => {
        //TODO: If cannot parse, should generate error w/ parsing error
        DID::parse($did).unwrap()
    };
}
