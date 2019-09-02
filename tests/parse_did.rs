use did_common::did::{Did, DidBuilder};

#[test]
fn parse_generic_did() {
    assert_eq!(
        Did::parse("did:example:1234"),
        Ok(DidBuilder::new("example", "1234").build())
    );
}

#[test]
fn parse_generic_did_with_empty_method_id() {
    assert_eq!(
        Did::parse("did:example:"),
        Ok(DidBuilder::new("example", "").build()));
}

#[test]
fn parse_generic_did_with_fragment() {
    let did = DidBuilder::new("example", "123456789abcdefghi")
        .with_fragment("keys-1")
        .build();

    assert_eq!(Did::parse("did:example:123456789abcdefghi#keys-1"), Ok(did))
}

#[test]
fn parse_btcr_did_with_key() {
    let expected = DidBuilder::new("btcr", "xyv2-xzpq-q9wa-p7t")
        .with_fragment("satoshi")
        .build();

    assert_eq!(
        Did::parse("did:btcr:xyv2-xzpq-q9wa-p7t#satoshi"),
        Ok(expected)
    )
}

#[test]
fn parse_ethr_did() {
    assert_eq!(
        Did::parse("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"),
        Ok(DidBuilder::new(
            "ethr",
            "0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"
        ).build())
    );
}

#[test]
fn parse_sovrin_did() {
    assert_eq!(
        Did::parse("did:sov:2wJPyULfLLnYTEFYzByfUR"),
        Ok(DidBuilder::new("sov", "2wJPyULfLLnYTEFYzByfUR").build())
    );
}

#[test]
fn parse_erc725_did() {
    assert_eq!(
        Did::parse("did:erc725:ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"),
        Ok(DidBuilder::new(
            "erc725",
            "ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"
        ).build())
    )
}

#[test]
fn parse_veres_one_did() {
    assert_eq!(
        Did::parse("did:v1:uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"),
        Ok(DidBuilder::new("v1", "uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74").build())
    )
}

#[test]
fn parse_did_with_generic_param() {
    assert_eq!(
        Did::parse("did:example:1234;service=agent"),
        Ok(DidBuilder::new("example","1234")
            .with_params(&[("service","agent")])
            .build()
        )
    );
}

#[test]
fn parse_did_with_method_specific_param() {
    assert_eq!(
        Did::parse("did:example:1234;example:foo:bar=baz"),
        Ok(DidBuilder::new("example","1234")
            .with_params(&[("example:foo:bar","baz")])
            .build()
        )
    );
}

#[test]
fn parse_did_with_multiple_params() {
    assert_eq!(
        Did::parse("did:example:1234;service=agent;example:foo:bar=baz"),
        Ok(DidBuilder::new("example","1234")
            .with_params(&[("service","agent"),("example:foo:bar","baz")])
            .build()
        )
    );
}

#[test]
fn parse_did_with_multiple_params_and_fragment() {
    assert_eq!(
        Did::parse("did:example:1234;service=agent;example:foo:bar=baz#keys-1"),
        Ok(DidBuilder::new("example","1234")
            .with_params(&[("service","agent"),("example:foo:bar","baz")])
            .with_fragment("keys-1")
            .build()
        )
    );
}
