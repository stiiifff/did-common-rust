use did_common::did::{Did, DidParam};

#[test]
fn parse_generic_did() {
    assert_eq!(
        Did::parse("did:example:1234"),
        Ok(Did::new("example", "1234"))
    );
}

#[test]
fn parse_generic_did_with_empty_method_id() {
    assert_eq!(Did::parse("did:example:"), Ok(Did::new("example", "")));
}

#[test]
fn parse_generic_did_with_fragment() {
    let mut did = Did::new("example", "123456789abcdefghi");
    did.fragment = Some("keys-1");

    assert_eq!(Did::parse("did:example:123456789abcdefghi#keys-1"), Ok(did))
}

#[test]
fn parse_btcr_did_with_key() {
    let mut expected = Did::new("btcr", "xyv2-xzpq-q9wa-p7t");
    expected.fragment = Some("satoshi");

    assert_eq!(
        Did::parse("did:btcr:xyv2-xzpq-q9wa-p7t#satoshi"),
        Ok(expected)
    )
}

#[test]
fn parse_ethr_did() {
    assert_eq!(
        Did::parse("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"),
        Ok(Did::new(
            "ethr",
            "0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"
        ))
    );
}

#[test]
fn parse_sovrin_did() {
    assert_eq!(
        Did::parse("did:sov:2wJPyULfLLnYTEFYzByfUR"),
        Ok(Did::new("sov", "2wJPyULfLLnYTEFYzByfUR"))
    );
}

#[test]
fn parse_erc725_did() {
    assert_eq!(
        Did::parse("did:erc725:ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"),
        Ok(Did::new(
            "erc725",
            "ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"
        ))
    )
}

#[test]
fn parse_veres_one_did() {
    assert_eq!(
        Did::parse("did:v1:uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"),
        Ok(Did::new("v1", "uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"))
    )
}

#[test]
fn parse_did_with_generic_param() {
    assert_eq!(
        Did::parse("did:example:1234;service=agent"),
        Ok(Did {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(DidParam {
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
        Did::parse("did:example:1234;example:foo:bar=baz"),
        Ok(Did {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(DidParam {
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
        Did::parse("did:example:1234;service=agent;example:foo:bar=baz"),
        Ok(Did {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(
                DidParam {
                    name: "service",
                    value: Some("agent")
                },
                DidParam {
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
        Did::parse("did:example:1234;service=agent;example:foo:bar=baz#keys-1"),
        Ok(Did {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(
                DidParam {
                    name: "service",
                    value: Some("agent")
                },
                DidParam {
                    name: "example:foo:bar",
                    value: Some("baz")
                }
            )),
            fragment: Some("keys-1")
        })
    );
}
