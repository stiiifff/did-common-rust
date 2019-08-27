use did_common::{did, DID, DIDParam};

#[test]
fn did_macro_generic_did() {
    assert_eq!(
        did!("did:example:1234"),
        DID::new("example", "1234")
    );
}

#[test]
fn did_macro_generic_id_with_empty_method_id() {
    assert_eq!(
        did!("did:example:"),
        DID::new("example", "")
    );
}

// #[test]
// fn did_macro_btcr_did_with_key() {
//     assert_eq!(
//         did_macro_did("did:btcr:xyv2-xzpq-q9wa-p7t#satoshi"),
//         Ok(("#satoshi", DID::new("btcr", "xyv2-xzpq-q9wa-p7t")))
//     )
// }

#[test]
fn did_macro_ethr_did() {
    assert_eq!(
        did!("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"),
        DID::new("ethr", "0xf3beac30c498d9e26865f34fcaa57dbb935b0d74")
    );
}

#[test]
fn did_macro_sovrin_did() {
    assert_eq!(
        did!("did:sov:2wJPyULfLLnYTEFYzByfUR"),
        DID::new("sov", "2wJPyULfLLnYTEFYzByfUR")
    );
}

#[test]
fn did_macro_erc725_did() {
    assert_eq!(
        did!("did:erc725:ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"),
        DID::new("erc725", "ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E")
    )
}

#[test]
fn did_macro_veres_one_did() {
    assert_eq!(
        did!("did:v1:uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"),
        DID::new("v1", "uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74")
    )
}

#[test]
fn did_macro_did_with_generic_param() {
    assert_eq!(
        did!("did:example:1234;service=agent"),
        DID {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(DIDParam {
                name: "service",
                value: Some("agent")
            }))
        }
    );
}

#[test]
fn did_macro_did_with_method_specific_param() {
    assert_eq!(
        did!("did:example:1234;example:foo:bar=baz"),
        DID {
            method_name: "example",
            method_specific_id: "1234",
            params: Some(vec!(DIDParam {
                name: "example:foo:bar",
                value: Some("baz")
            }))
        }
    );
}

#[test]
fn did_macro_did_with_multiple_params() {
    assert_eq!(
        did!("did:example:1234;service=agent;example:foo:bar=baz"),
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
    );
}
