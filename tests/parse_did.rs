use did_common::{DID, DIDParam};

#[test]
fn parse_generic_did() {
    assert_eq!(
        DID::parse("did:example:1234"),
        Ok(DID::new("example", "1234"))
    );
}

#[test]
fn parse_generic_did_with_empty_method_id() {
    assert_eq!(
        DID::parse("did:example:"),
        Ok(DID::new("example", ""))
    );
}

#[test]
fn parse_generic_did_with_fragment() {
    let mut did = DID::new("example", "123456789abcdefghi");
    did.fragment = Some("keys-1");

    assert_eq!(
        DID::parse("did:example:123456789abcdefghi#keys-1"),
        Ok(did)
    )
}

#[test]
fn parse_btcr_did_with_key() {
    let mut expected = DID::new("btcr", "xyv2-xzpq-q9wa-p7t");
    expected.fragment = Some("satoshi");

    assert_eq!(
        DID::parse("did:btcr:xyv2-xzpq-q9wa-p7t#satoshi"),
        Ok(expected)
    )
}

#[test]
fn parse_ethr_did() {
    assert_eq!(
        DID::parse("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"),
        Ok(DID::new("ethr", "0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"))
    );
}

#[test]
fn parse_sovrin_did() {
    assert_eq!(
        DID::parse("did:sov:2wJPyULfLLnYTEFYzByfUR"),
        Ok(DID::new("sov", "2wJPyULfLLnYTEFYzByfUR"))
    );
}

#[test]
fn parse_erc725_did() {
    assert_eq!(
        DID::parse("did:erc725:ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"),
        Ok(DID::new("erc725", "ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"))
    )
}

#[test]
fn parse_veres_one_did() {
    assert_eq!(
        DID::parse("did:v1:uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"),
        Ok(DID::new("v1", "uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"))
    )
}

#[test]
fn parse_did_with_generic_param() {
    assert_eq!(
        DID::parse("did:example:1234;service=agent"),
        Ok(
            DID {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(DIDParam {
                name: "service",
                value: Some("agent")
            })),
            fragment: None
        })
    );
}

#[test]
fn parse_did_with_method_specific_param() {
    assert_eq!(
        DID::parse("did:example:1234;example:foo:bar=baz"),
        Ok(
            DID {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(DIDParam {
                name: "example:foo:bar",
                value: Some("baz")
            })),
            fragment: None
        })
    );
}

#[test]
fn parse_did_with_multiple_params() {
    assert_eq!(
        DID::parse("did:example:1234;service=agent;example:foo:bar=baz"),
        Ok(
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
            fragment: None
        })
    );
}

#[test]
fn parse_did_with_multiple_params_and_fragment() {
    assert_eq!(
        DID::parse("did:example:1234;service=agent;example:foo:bar=baz#keys-1"),
        Ok(
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
            fragment: Some("keys-1")
        })
    );
}
