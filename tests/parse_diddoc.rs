use did_common::did_doc::{
	DidDocument, DidDocumentBuilder, PublicKeyBuilder, PublicKeyEncoded, PublicKeyType, Service,
	ServiceEndpoint, VerificationMethod,
};

fn json_parse(input: &str) -> json::JsonValue {
	did_common::json_parse(input).unwrap()
}

#[test]
fn parse_did_doc_with_missing_context() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#
		)),
		Err("missing DID context")
	);
}

#[test]
fn parse_did_doc_with_invalid_context() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://w3id.org/security/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#
		)),
		Err("invalid DID context")
	);
}

#[test]
fn parse_did_doc_with_missing_subject() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1"
        }
        "#
		)),
		Err("missing DID subject")
	);
}

#[test]
fn parse_did_doc_with_invalid_subject() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "foobar"
        }
        "#
		)),
		Err("invalid DID subject")
	);
}

#[test]
fn parse_minimal_did_doc() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp"
        }
        "#
		)),
		Ok(DidDocumentBuilder::new("did:example:21tDAKCERh95uGgKbJNHYp").build())
	);
}

#[test]
fn parse_did_doc_with_invalid_created() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp",
			"created": "2002-10-32T17:00:00"
        }
        "#
		)),
		Err("invalid created timestamp")
	);
}

#[test]
fn parse_did_doc_with_created() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp",
			"created": "2002-10-10T17:00:00Z"
        }
        "#
		)),
		Ok(
			DidDocumentBuilder::new("did:example:21tDAKCERh95uGgKbJNHYp")
				.created_on("2002-10-10T17:00:00Z")
				.build()
		)
	);
}

#[test]
fn parse_did_doc_with_invalid_updated() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp",
			"updated": "2002-10-32T17:00:00"
        }
        "#
		)),
		Err("invalid updated timestamp")
	);
}

#[test]
fn parse_did_doc_with_updated() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:21tDAKCERh95uGgKbJNHYp",
			"updated": "2002-10-10T17:00:00Z"
        }
        "#
		)),
		Ok(
			DidDocumentBuilder::new("did:example:21tDAKCERh95uGgKbJNHYp")
				.updated_on("2002-10-10T17:00:00Z")
				.build()
		)
	);
}

#[test]
fn parse_did_doc_with_pub_keys() {
	assert_eq!(
        DidDocument::parse(&json_parse(
            r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:123456789abcdefghi",
            "publicKey": [
                {
                    "id": "did:example:123456789abcdefghi#keys-1",
                    "type": "RsaVerificationKey2018",
                    "controller": "did:example:123456789abcdefghi",
                    "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
                }, {
                    "id": "did:example:123456789abcdefghi#keys-2",
                    "type": "Ed25519VerificationKey2018",
                    "controller": "did:example:pqrstuvwxyz0987654321",
                    "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
                }, {
                    "id": "did:example:123456789abcdefghi#keys-3",
                    "type": "Secp256k1VerificationKey2018",
                    "controller": "did:example:123456789abcdefghi",
                    "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
                }
            ]
        }
        "#
        )),
        Ok(
            DidDocumentBuilder::new("did:example:123456789abcdefghi")
                .with_pubkeys(vec![
                    PublicKeyBuilder::new(
                        "did:example:123456789abcdefghi#keys-1",
                        PublicKeyType::Rsa,
                        "did:example:123456789abcdefghi"
                    )
                    .with_encoded_key(PublicKeyEncoded::Pem(
                        "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
                    ))
                    .build(),
                    PublicKeyBuilder::new(
                        "did:example:123456789abcdefghi#keys-2",
                        PublicKeyType::Ed25519,
                        "did:example:pqrstuvwxyz0987654321"
                    )
                    .with_encoded_key(PublicKeyEncoded::Base58(
                        "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
                    ))
                    .build(),
                    PublicKeyBuilder::new(
                        "did:example:123456789abcdefghi#keys-3",
                        PublicKeyType::EcdsaSecp256k1,
                        "did:example:123456789abcdefghi"
                    )
                    .with_encoded_key(PublicKeyEncoded::Hex(
                        "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
                    ))
                    .build()
                ])
                .build()
        )
    );
}

#[test]
fn parse_did_doc_with_auth_reference_verif_method() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:123456789abcdefghi",
            "authentication": [ "did:example:123456789abcdefghi#keys-1" ],
			"publicKey": [
                {
                    "id": "did:example:123456789abcdefghi#keys-1",
                    "type": "RsaVerificationKey2018",
                    "controller": "did:example:123456789abcdefghi",
                    "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
                }
            ]
        }
        "#
		)),
		Ok(DidDocumentBuilder::new("did:example:123456789abcdefghi")
			.with_authentication(vec![VerificationMethod::Reference(
				"did:example:123456789abcdefghi#keys-1"
			)])
			.with_pubkeys(vec![PublicKeyBuilder::new(
				"did:example:123456789abcdefghi#keys-1",
				PublicKeyType::Rsa,
				"did:example:123456789abcdefghi"
			)
			.with_encoded_key(PublicKeyEncoded::Pem(
				"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
			))
			.build()])
			.build())
	);
}

#[test]
fn parse_did_doc_with_unknown_auth_reference() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:123456789abcdefghi",
            "authentication": [ "did:example:123456789abcdefghi#keys-1" ]
        }
        "#
		)),
		Err("unknown reference verification method")
	);
}

#[test]
fn parse_did_doc_with_auth_embedded_verif_method() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:123456789abcdefghi",
            "authentication": [
                {
                    "id": "did:example:123456789abcdefghi#keys-2",
                    "type": "Ed25519VerificationKey2018",
                    "controller": "did:example:123456789abcdefghi",
                    "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
                }
            ]
        }
        "#
		)),
		Ok(DidDocumentBuilder::new("did:example:123456789abcdefghi")
			.with_authentication(vec![VerificationMethod::Embedded(
				PublicKeyBuilder::new(
					"did:example:123456789abcdefghi#keys-2",
					PublicKeyType::Ed25519,
					"did:example:123456789abcdefghi"
				)
				.with_encoded_key(PublicKeyEncoded::Base58(
					"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
				))
				.build(),
			)])
			.build())
	);
}

#[test]
fn parse_did_doc_with_duplicate_id_from_auth_embedded() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:123456789abcdefghi",
            "authentication": [
                {
                    "id": "did:example:123456789abcdefghi#keys-2",
                    "type": "Ed25519VerificationKey2018",
                    "controller": "did:example:123456789abcdefghi",
                    "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
                }
            ],
			"publicKey": [
                {
                    "id": "did:example:123456789abcdefghi#keys-2",
                    "type": "Ed25519VerificationKey2018",
                    "controller": "did:example:pqrstuvwxyz0987654321",
                    "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
                }
            ]
        }
        "#
		)),
		Err("duplicate public key id from embedded verification method")
	);
}

#[test]
fn parse_did_doc_with_service_uri_endpoints() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:123456789abcdefghi",
			"service": [{
				"id": "did:example:123456789abcdefghi#openid",
				"type": "OpenIdConnectVersion1.0Service",
				"serviceEndpoint": "https://openid.example.com/"
			}, {
				"id": "did:example:123456789abcdefghi#vcr",
				"type": "CredentialRepositoryService",
				"serviceEndpoint": "https://repository.example.com/service/8377464"
			}, {
				"id": "did:example:123456789abcdefghi#xdi",
				"type": "XdiService",
				"serviceEndpoint": "https://xdi.example.com/8377464"
			}, {
				"id": "did:example:123456789abcdefghi#agent",
				"type": "AgentService",
				"serviceEndpoint": "https://agent.example.com/8377464"
			}]
        }
        "#
		)),
		Ok(DidDocumentBuilder::new("did:example:123456789abcdefghi")
			.with_services(vec![
				Service::new(
					"did:example:123456789abcdefghi#openid",
					"OpenIdConnectVersion1.0Service",
					ServiceEndpoint::Uri("https://openid.example.com/")
				),
				Service::new(
					"did:example:123456789abcdefghi#vcr",
					"CredentialRepositoryService",
					ServiceEndpoint::Uri("https://repository.example.com/service/8377464")
				),
				Service::new(
					"did:example:123456789abcdefghi#xdi",
					"XdiService",
					ServiceEndpoint::Uri("https://xdi.example.com/8377464")
				),
				Service::new(
					"did:example:123456789abcdefghi#agent",
					"AgentService",
					ServiceEndpoint::Uri("https://agent.example.com/8377464")
				),
			])
			.build())
	);
}

#[test]
fn parse_did_doc_with_service_object_endpoint() {
	assert_eq!(
		DidDocument::parse(&json_parse(
			r#"
        {
            "@context": "https://www.w3.org/2019/did/v1",
            "id": "did:example:123456789abcdefghi",
			"service": [
				{
					"id": "did:example:123456789abcdefghi#hub",
					"type": "IdentityHub",
					"publicKey": "did:example:123456789abcdefghi#key-1",
					"serviceEndpoint": {
						"@context": "https://schema.identity.foundation/hub",
						"type": "UserHubEndpoint",
						"instances": ["did:example:456", "did:example:789"]
					}
				}
			]
        }
        "#
		)),
		Err("invalid service endpoint JSON-LD object : unimplemented")
	);
}
