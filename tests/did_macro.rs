use did_common::did;
use did_common::did::{Did, DidBuilder};

#[test]
fn did_macro_generic_did() {
    assert_eq!(
        did!("did:example:1234"),
        DidBuilder::new("example", "1234").build());
}

#[test]
fn did_macro_generic_did_with_empty_method_id() {
    assert_eq!(
        did!("did:example:"),
        DidBuilder::new("example", "").build());
}

#[test]
fn did_macro_generic_did_with_fragment() {
    let did = DidBuilder::new("example", "123456789abcdefghi")
        .with_fragment("keys-1")
        .build();

    assert_eq!(did!("did:example:123456789abcdefghi#keys-1"), did)
}

#[test]
fn did_macro_btcr_did_with_key() {
    let did = DidBuilder::new("btcr", "xyv2-xzpq-q9wa-p7t")
        .with_fragment("satoshi")
        .build();

    assert_eq!(did!("did:btcr:xyv2-xzpq-q9wa-p7t#satoshi"), did)
}

#[test]
fn did_macro_ethr_did() {
    assert_eq!(
        did!("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"),
        DidBuilder::new("ethr", "0xf3beac30c498d9e26865f34fcaa57dbb935b0d74").build()
    );
}

#[test]
fn did_macro_sovrin_did() {
    assert_eq!(
        did!("did:sov:2wJPyULfLLnYTEFYzByfUR"),
        DidBuilder::new("sov", "2wJPyULfLLnYTEFYzByfUR").build()
    );
}

#[test]
fn did_macro_erc725_did() {
    assert_eq!(
        did!("did:erc725:ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"),
        DidBuilder::new("erc725", "ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E").build()
    )
}

#[test]
fn did_macro_veres_one_did() {
    assert_eq!(
        did!("did:v1:uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"),
        DidBuilder::new("v1", "uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74").build()
    )
}

#[test]
fn did_macro_did_with_generic_param() {
    assert_eq!(
        did!("did:example:1234;service=agent"),
        DidBuilder::new("example","1234")
            .with_params(&[("service","agent")])
            .build()
    );
}

#[test]
fn did_macro_did_with_method_specific_param() {
    assert_eq!(
        did!("did:example:1234;example:foo:bar=baz"),
        DidBuilder::new("example","1234")
            .with_params(&[("example:foo:bar","baz")])
            .build()
    );
}

#[test]
fn did_macro_did_with_multiple_params() {
    assert_eq!(
        did!("did:example:1234;service=agent;example:foo:bar=baz"),
        DidBuilder::new("example","1234")
            .with_params(&[("service","agent"),("example:foo:bar","baz")])
            .build()
    );
}

#[test]
fn did_macro_did_with_multiple_params_and_fragment() {
    assert_eq!(
        did!("did:example:1234;service=agent;example:foo:bar=baz#keys-1"),
        DidBuilder::new("example","1234")
            .with_params(&[("service","agent"),("example:foo:bar","baz")])
            .with_fragment("keys-1")
            .build()
    );
}
