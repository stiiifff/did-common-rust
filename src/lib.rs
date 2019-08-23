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

#[cfg(test)]
mod tests {
    use super::{DID, DIDParam};
    use super::did_parser::parse_did;

    #[test]
    fn parse_generic_did() {
        assert_eq!(
            parse_did("did:example:1234"),
            Ok(("", DID::new("example", "1234")))
        );
    }

    #[test]
    fn parse_generic_id_with_empty_method_id() {
        assert_eq!(parse_did("did:example:"), Ok(("", DID::new("example", ""))));
    }

    #[test]
    fn parse_btcr_did_with_key() {
        assert_eq!(
            parse_did("did:btcr:xyv2-xzpq-q9wa-p7t#satoshi"),
            Ok(("#satoshi", DID::new("btcr", "xyv2-xzpq-q9wa-p7t")))
        )
    }

    #[test]
    fn parse_ethr_did() {
        assert_eq!(
            parse_did("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"),
            Ok((
                "",
                DID::new("ethr", "0xf3beac30c498d9e26865f34fcaa57dbb935b0d74")
            ))
        );
    }

    #[test]
    fn parse_sovrin_did() {
        assert_eq!(
            parse_did("did:sov:2wJPyULfLLnYTEFYzByfUR"),
            Ok(("", DID::new("sov", "2wJPyULfLLnYTEFYzByfUR")))
        );
    }

    #[test]
    fn parse_erc725_did() {
        assert_eq!(
            parse_did("did:erc725:ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"),
            Ok((
                "",
                DID::new("erc725", "ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E")
            ))
        )
    }

    #[test]
    fn parse_veres_one_did() {
        assert_eq!(
            parse_did("did:v1:uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"),
            Ok((
                "",
                DID::new("v1", "uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74")
            ))
        )
    }

    #[test]
    fn parse_did_with_generic_param() {
        assert_eq!(
            parse_did("did:example:1234;service=agent"),
            Ok((
                "",
                DID {
                    method_name: "example",
                    method_specific_id: "1234",
                    params: Some(vec!(DIDParam {
                        name: "service",
                        value: Some("agent")
                    }))
                }
            ))
        );
    }

    #[test]
    fn parse_did_with_method_specific_param() {
        assert_eq!(
            parse_did("did:example:1234;example:foo:bar=baz"),
            Ok((
                "",
                DID {
                    method_name: "example",
                    method_specific_id: "1234",
                    params: Some(vec!(DIDParam {
                        name: "example:foo:bar",
                        value: Some("baz")
                    }))
                }
            ))
        );
    }

    #[test]
    fn parse_did_with_multiple_params() {
        assert_eq!(
            parse_did("did:example:1234;service=agent;example:foo:bar=baz"),
            Ok((
                "",
                DID {
                    method_name: "example",
                    method_specific_id: "1234",
                    params: Some(vec!(
                        DIDParam {
                            name: "service",
                            value: Some("agent")
                        },
                        DIDParam {
                            name: "example:foo:bar",
                            value: Some("baz")
                        }
                    )),
                }
            ))
        );
    }
}
