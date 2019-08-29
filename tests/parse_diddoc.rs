use did_common::did_doc::{DidDocument, PublicKey, PublicKeyEncoded, PublicKeyType};

#[test]
fn parse_did_doc_with_missing_context() {
    assert_eq!(
        DidDocument::parse(
            r#"
        {
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#
        ),
        Err("missing DID context")
    );
}

#[test]
fn parse_did_doc_with_invalid_context() {
    assert_eq!(
        DidDocument::parse(
            r#"
        {
            "@context": "https://w3id.org/security/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#
        ),
        Err("invalid DID context")
    );
}

#[test]
fn parse_did_doc_with_missing_subject() {
    assert_eq!(
        DidDocument::parse(
            r#"
        {
            "@context": "https://www.w3.org/2019/did/v1"
        }
        "#
        ),
        Err("missing DID subject")
    );
}

#[test]
fn parse_did_doc_with_invalid_subject() {
    assert_eq!(
        DidDocument::parse(
            r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "foobar"
        }
        "#
        ),
        Err("invalid DID subject")
    );
}

#[test]
fn parse_minimal_did_doc() {
    assert_eq!(
        DidDocument::parse(
            r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#
        ),
        Ok(DidDocument::new(
            "did:example:21tDAKCERh95uGgKbJNHYp".to_string()
        ))
    );
}

#[test]
fn parse_did_doc_with_ed25519_pubkey() {
    assert_eq!(
        DidDocument::parse(
            r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp",
            "publicKey": [{
                "id": "did:example:123456789abcdefghi#keys-1",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:example:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }]
        }
        "#
        ),
        Ok(DidDocument::with_pubkeys(
            "did:example:21tDAKCERh95uGgKbJNHYp".to_string(),
            vec![PublicKey {
                id: "did:example:123456789abcdefghi#keys-1".to_string(),
                r#type: PublicKeyType::Ed25519,
                controller: "did:example:pqrstuvwxyz0987654321".to_string(),
                pub_key: PublicKeyEncoded::Base58(
                    "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".to_string()
                )
            }]
        ))
    );
}
