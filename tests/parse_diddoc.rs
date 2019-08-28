use did_common::{DIDDocument, PublicKey, PublicKeyType, PublicKeyFormat};

#[test]
fn parse_did_doc_with_missing_context() {
    assert_eq!(
        DIDDocument::parse(r#"
        {
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#),
        Err("missing DID context")
    );
}

#[test]
fn parse_did_doc_with_invalid_context() {
    assert_eq!(
        DIDDocument::parse(r#"
        {
            "@context": "https://w3id.org/security/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#),
        Err("invalid DID context")
    );
}

#[test]
fn parse_did_doc_with_missing_subject() {
    assert_eq!(
        DIDDocument::parse(r#"
        {
            "@context": "https://www.w3.org/2019/did/v1"
        }
        "#),
        Err("missing DID subject")
    );
}

#[test]
fn parse_did_doc_with_invalid_subject() {
    assert_eq!(
        DIDDocument::parse(r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "foobar"
        }
        "#),
        Err("invalid DID subject")
    );
}

#[test]
fn parse_minimal_did_doc() {
    assert_eq!(
        DIDDocument::parse(r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#),
        Ok(DIDDocument::new("did:example:21tDAKCERh95uGgKbJNHYp".to_string()))
    );
}

#[test]
fn parse_did_doc_with_ed25519_pubkey() {
    assert_eq!(
        DIDDocument::parse(r#"
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
        "#),
        Ok(DIDDocument::with_pubkeys("did:example:21tDAKCERh95uGgKbJNHYp".to_string(),
            vec![
                PublicKey {
                    id: "", r#type: PublicKeyType::Rsa, controller: "",
                    pub_key: PublicKeyFormat::Pem("-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n".to_string())
                }
            ]))
    );
}
