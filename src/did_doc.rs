use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::diddoc_parser;
use json::JsonValue;

#[derive(Clone, Debug, PartialEq)]
pub enum PublicKeyType {
	Rsa,
	Ed25519,
	EcdsaSecp256k1,
}

impl FromStr for PublicKeyType {
	type Err = ParsePublicKeyTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"RsaVerificationKey2018" => Ok(Self::Rsa),
			"Ed25519VerificationKey2018" => Ok(Self::Ed25519),
			"Secp256k1VerificationKey2018" => Ok(Self::EcdsaSecp256k1),
			_ => Result::Err(ParsePublicKeyTypeError(())),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsePublicKeyTypeError(());

impl fmt::Display for ParsePublicKeyTypeError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.write_str(self.description())
	}
}

impl Error for ParsePublicKeyTypeError {
	fn description(&self) -> &str {
		"invalid DID public key type"
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum PublicKeyEncoded<'a> {
	None,
	Pem(&'a str),
	Jwk(&'a str),
	Hex(&'a str),
	Base64(&'a str),
	Base58(&'a str),
	Multibase(&'a str),
	EthrAddress(&'a str),
	Unsupported,
}

const KEYPEM_PROP: &str = "publicKeyPem";
const KEYJWK_PROP: &str = "publicKeyJwk";
const KEYHEX_PROP: &str = "publicKeyHex";
const KEYB58_PROP: &str = "publicKeyBase58";
const KEYB64_PROP: &str = "publicKeyBase64";
const KEYMUL_PROP: &str = "publicKeyMultibase";
const KEYETH_PROP: &str = "ethereumAddress";
pub const KEY_FORMATS: [&str; 7] = [
	KEYPEM_PROP,
	KEYJWK_PROP,
	KEYHEX_PROP,
	KEYB58_PROP,
	KEYB64_PROP,
	KEYMUL_PROP,
	KEYETH_PROP,
];

impl<'a> From<(&'a str, &'a str)> for PublicKeyEncoded<'a> {
	fn from(s: (&'a str, &'a str)) -> Self {
		match s.0 {
			KEYPEM_PROP => PublicKeyEncoded::Pem(s.1),
			KEYJWK_PROP => PublicKeyEncoded::Jwk(s.1),
			KEYHEX_PROP => PublicKeyEncoded::Hex(s.1),
			KEYB58_PROP => PublicKeyEncoded::Base58(s.1),
			KEYB64_PROP => PublicKeyEncoded::Base64(s.1),
			KEYMUL_PROP => PublicKeyEncoded::Multibase(s.1),
			KEYETH_PROP => PublicKeyEncoded::EthrAddress(s.1),
			_ => PublicKeyEncoded::Unsupported,
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct PublicKey<'a> {
	id: &'a str,
	key_type: PublicKeyType,
	controller: &'a str,
	encoded_key: PublicKeyEncoded<'a>,
}

impl<'a> PublicKey<'a> {
	pub fn id(&self) -> &'a str {
		self.id
	}

	pub fn key_type(&self) -> &PublicKeyType {
		&self.key_type
	}

	pub fn controller(&self) -> &'a str {
		self.controller
	}

	pub fn encoded_key(&self) -> &PublicKeyEncoded {
		&self.encoded_key
	}
}

#[derive(Debug, PartialEq)]
pub struct PublicKeyBuilder<'a> {
	id: &'a str,
	key_type: PublicKeyType,
	controller: &'a str,
	encoded_key: PublicKeyEncoded<'a>,
}

impl<'a> PublicKeyBuilder<'a> {
	pub fn new(id: &'a str, key_type: PublicKeyType, controller: &'a str) -> Self {
		PublicKeyBuilder {
			id,
			key_type,
			controller,
			encoded_key: PublicKeyEncoded::None,
		}
	}

	pub fn with_encoded_key(mut self, encoded_key: PublicKeyEncoded<'a>) -> Self {
		self.encoded_key = encoded_key;
		self
	}

	pub fn build(self) -> PublicKey<'a> {
		PublicKey {
			id: self.id,
			key_type: self.key_type,
			controller: self.controller,
			encoded_key: self.encoded_key,
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum VerificationMethod<'a> {
	Reference(&'a str),
	Embedded(PublicKey<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ServiceEndpoint<'a> {
	Uri(&'a str),
	Object(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Service<'a> {
	id: &'a str,
	svc_type: &'a str,
	endpoint: ServiceEndpoint<'a>,
}

impl<'a> Service<'a> {
	pub fn new(id: &'a str, svc_type: &'a str, endpoint: ServiceEndpoint<'a>) -> Self {
		Service {
			id,
			svc_type,
			endpoint,
		}
	}

	pub fn id(&self) -> &'a str {
		self.id
	}

	pub fn svc_type(&self) -> &'a str {
		self.svc_type
	}

	pub fn endpoint(&self) -> &ServiceEndpoint {
		&self.endpoint
	}
}

#[derive(Debug, Default, PartialEq)]
pub struct DidDocument<'a> {
	context: &'a str,
	id: &'a str,
	created: Option<&'a str>,
	updated: Option<&'a str>,
	authentication: Vec<VerificationMethod<'a>>,
	pub_keys: Vec<PublicKey<'a>>,
	service: Vec<Service<'a>>,
}

impl<'a> DidDocument<'a> {
	pub fn context(&self) -> &'a str {
		self.context
	}

	pub fn id(&self) -> &'a str {
		self.id
	}

	pub fn created(&self) -> Option<&'a str> {
		self.created
	}

	pub fn updated(&self) -> Option<&'a str> {
		self.updated
	}

	pub fn authentication(&self) -> &[VerificationMethod<'a>] {
		&self.authentication[..]
	}

	pub fn pub_keys(&self) -> &[PublicKey<'a>] {
		&self.pub_keys[..]
	}

	pub fn service(&self) -> &[Service<'a>] {
		&self.service[..]
	}

	pub fn parse(json: &'a JsonValue) -> Result<Self, &'a str> {
		diddoc_parser::parse_did_doc(json)
	}
}

#[derive(Debug, Default, PartialEq)]
pub struct DidDocumentBuilder<'a> {
	context: &'a str,
	id: &'a str,
	created: Option<&'a str>,
	updated: Option<&'a str>,
	authentication: Vec<VerificationMethod<'a>>,
	pub_keys: Vec<PublicKey<'a>>,
	service: Vec<Service<'a>>,
}

impl<'a> DidDocumentBuilder<'a> {
	pub fn new(id: &'a str) -> Self {
		DidDocumentBuilder {
			context: diddoc_parser::GENERIC_DID_CTX,
			id,
			..Default::default()
		}
	}

	pub fn created_on(mut self, created: &'a str) -> Self {
		self.created = Some(created);
		self
	}

	pub fn updated_on(mut self, updated: &'a str) -> Self {
		self.updated = Some(updated);
		self
	}

	pub fn with_authentication(
		mut self,
		authentication: std::vec::Vec<VerificationMethod<'a>>,
	) -> Self {
		self.authentication = authentication;
		self
	}

	pub fn with_pubkeys(mut self, pub_keys: Vec<PublicKey<'a>>) -> Self {
		self.pub_keys = pub_keys;
		self
	}

	pub fn with_services(mut self, services: Vec<Service<'a>>) -> Self {
		self.service = services;
		self
	}

	pub fn build(self) -> DidDocument<'a> {
		DidDocument {
			context: self.context,
			id: self.id,
			created: self.created,
			updated: self.updated,
			authentication: self.authentication,
			pub_keys: self.pub_keys,
			service: self.service,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::diddoc_parser::GENERIC_DID_CTX;
	use super::{
		DidDocument, DidDocumentBuilder, ParsePublicKeyTypeError, PublicKey, PublicKeyBuilder,
		PublicKeyEncoded, PublicKeyType, Service, ServiceEndpoint, VerificationMethod,
	};
	use std::str::FromStr;

	const TEST_ENCODED_KEY: &str = "0x1234567890";

	#[test]
	fn public_key_type_fromstr_trait_for_rsa() {
		assert_eq!(
			PublicKeyType::from_str("RsaVerificationKey2018"),
			Ok(PublicKeyType::Rsa)
		);
	}

	#[test]
	fn public_key_type_fromstr_trait_for_ed25519() {
		assert_eq!(
			PublicKeyType::from_str("Ed25519VerificationKey2018"),
			Ok(PublicKeyType::Ed25519)
		);
	}

	#[test]
	fn public_key_type_fromstr_trait_for_ecdsasecp256k1() {
		assert_eq!(
			PublicKeyType::from_str("Secp256k1VerificationKey2018"),
			Ok(PublicKeyType::EcdsaSecp256k1)
		);
	}

	#[test]
	fn public_key_type_fromstr_trait_for_invalid_case() {
		assert_eq!(
			PublicKeyType::from_str("rsaverificationkey2018"),
			Err(ParsePublicKeyTypeError(()))
		);
		assert_eq!(
			PublicKeyType::from_str("ed25519verificationkey2018"),
			Err(ParsePublicKeyTypeError(()))
		);
		assert_eq!(
			PublicKeyType::from_str("secp256k1verificationkey2018"),
			Err(ParsePublicKeyTypeError(()))
		);
	}

	#[test]
	fn public_key_type_fromstr_trait_for_unsupported() {
		assert_eq!(
			PublicKeyType::from_str("azertyuiop"),
			Err(ParsePublicKeyTypeError(()))
		);
	}

	#[test]
	fn public_key_type_error_display_trait() {
		assert_eq!(
			format!("{}", ParsePublicKeyTypeError(())),
			"invalid DID public key type"
		);
	}

	#[test]
	fn public_key_encoded_from_trait_for_pem() {
		assert_eq!(
			PublicKeyEncoded::from((super::KEYPEM_PROP, TEST_ENCODED_KEY)),
			PublicKeyEncoded::Pem(TEST_ENCODED_KEY)
		)
	}

	#[test]
	fn public_key_encoded_from_trait_for_jwk() {
		assert_eq!(
			PublicKeyEncoded::from((super::KEYJWK_PROP, TEST_ENCODED_KEY)),
			PublicKeyEncoded::Jwk(TEST_ENCODED_KEY)
		)
	}

	#[test]
	fn public_key_encoded_from_trait_for_hex() {
		assert_eq!(
			PublicKeyEncoded::from((super::KEYHEX_PROP, TEST_ENCODED_KEY)),
			PublicKeyEncoded::Hex(TEST_ENCODED_KEY)
		)
	}

	#[test]
	fn public_key_encoded_from_trait_for_base64() {
		assert_eq!(
			PublicKeyEncoded::from((super::KEYB64_PROP, TEST_ENCODED_KEY)),
			PublicKeyEncoded::Base64(TEST_ENCODED_KEY)
		)
	}

	#[test]
	fn public_key_encoded_from_trait_for_base58() {
		assert_eq!(
			PublicKeyEncoded::from((super::KEYB58_PROP, TEST_ENCODED_KEY)),
			PublicKeyEncoded::Base58(TEST_ENCODED_KEY)
		)
	}

	#[test]
	fn public_key_encoded_from_trait_for_multibase() {
		assert_eq!(
			PublicKeyEncoded::from((super::KEYMUL_PROP, TEST_ENCODED_KEY)),
			PublicKeyEncoded::Multibase(TEST_ENCODED_KEY)
		)
	}

	#[test]
	fn public_key_encoded_from_trait_for_ethraddress() {
		assert_eq!(
			PublicKeyEncoded::from((super::KEYETH_PROP, TEST_ENCODED_KEY)),
			PublicKeyEncoded::EthrAddress(TEST_ENCODED_KEY)
		)
	}

	#[test]
	fn public_key_encoded_from_trait_for_unsupported() {
		assert_eq!(
			PublicKeyEncoded::from(("azertyuiop", TEST_ENCODED_KEY)),
			PublicKeyEncoded::Unsupported
		)
	}

	#[test]
	fn public_key_property_accessors() {
		let pubkey = PublicKey {
			id: "did:example:123456789abcdefghi#keys-1",
			key_type: PublicKeyType::Ed25519,
			controller: "did:example:123456789abcdefghi",
			encoded_key: PublicKeyEncoded::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"),
		};
		assert_eq!(pubkey.id(), "did:example:123456789abcdefghi#keys-1");
		assert_eq!(pubkey.key_type(), &PublicKeyType::Ed25519);
		assert_eq!(pubkey.controller(), "did:example:123456789abcdefghi");
		assert_eq!(
			pubkey.encoded_key(),
			&PublicKeyEncoded::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV")
		);
	}

	#[test]
	fn public_key_builder_for_rsa_key() {
		assert_eq!(
			PublicKeyBuilder::new(
				"did:example:123456789abcdefghi#keys-1",
				PublicKeyType::Rsa,
				"did:example:123456789abcdefghi"
			)
			.with_encoded_key(PublicKeyEncoded::Pem(
				"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
			))
			.build(),
			PublicKey {
				id: "did:example:123456789abcdefghi#keys-1",
				key_type: PublicKeyType::Rsa,
				controller: "did:example:123456789abcdefghi",
				encoded_key: PublicKeyEncoded::Pem(
					"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
				),
			}
		)
	}

	#[test]
	fn public_key_builder_for_ed25519_key() {
		assert_eq!(
			PublicKeyBuilder::new(
				"did:example:123456789abcdefghi#keys-2",
				PublicKeyType::Ed25519,
				"did:example:pqrstuvwxyz0987654321"
			)
			.with_encoded_key(PublicKeyEncoded::Base58(
				"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
			))
			.build(),
			PublicKey {
				id: "did:example:123456789abcdefghi#keys-2",
				key_type: PublicKeyType::Ed25519,
				controller: "did:example:pqrstuvwxyz0987654321",
				encoded_key: PublicKeyEncoded::Base58(
					"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
				),
			}
		)
	}

	#[test]
	fn public_key_builder_for_edcsasecp256k1_key() {
		assert_eq!(
			PublicKeyBuilder::new(
				"did:example:123456789abcdefghi#keys-3",
				PublicKeyType::EcdsaSecp256k1,
				"did:example:123456789abcdefghi"
			)
			.with_encoded_key(PublicKeyEncoded::Hex(
				"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
			))
			.build(),
			PublicKey {
				id: "did:example:123456789abcdefghi#keys-3",
				key_type: PublicKeyType::EcdsaSecp256k1,
				controller: "did:example:123456789abcdefghi",
				encoded_key: PublicKeyEncoded::Hex(
					"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
				),
			}
		)
	}

	#[test]
	fn service_property_accessors() {
		let svc = Service::new(
			"did:example:123456789abcdefghi#openid",
			"OpenIdConnectVersion1.0Service",
			ServiceEndpoint::Uri("https://openid.example.com/"),
		);
		assert_eq!(svc.id(), "did:example:123456789abcdefghi#openid");
		assert_eq!(svc.svc_type(), "OpenIdConnectVersion1.0Service");
		assert_eq!(
			svc.endpoint(),
			&ServiceEndpoint::Uri("https://openid.example.com/")
		);
	}

	#[test]
	fn did_document_property_accessors() {
		let pubkey = PublicKey {
			id: "did:example:123456789abcdefghi#keys-1",
			key_type: PublicKeyType::Ed25519,
			controller: "did:example:123456789abcdefghi",
			encoded_key: PublicKeyEncoded::Base58("H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"),
		};

		let verif_method = VerificationMethod::Embedded(pubkey.clone());

		let service = Service {
			id: "did:example:123456789abcdefghi#openid",
			svc_type: "OpenIdConnectVersion1.0Service",
			endpoint: ServiceEndpoint::Uri("https://openid.example.com/"),
		};

		let did_doc = DidDocument {
			context: "https://www.w3.org/2019/did/v1",
			id: "did:example:123456789abcdefghi",
			created: Some("2002-10-10T17:00:00Z"),
			updated: Some("2002-10-10T17:00:00Z"),
			authentication: vec![verif_method.clone()],
			pub_keys: vec![pubkey.clone()],
			service: vec![service.clone()],
		};
		assert_eq!(did_doc.context(), "https://www.w3.org/2019/did/v1");
		assert_eq!(did_doc.id(), "did:example:123456789abcdefghi");
		assert_eq!(did_doc.created(), Some("2002-10-10T17:00:00Z"));
		assert_eq!(did_doc.created(), Some("2002-10-10T17:00:00Z"));
		assert_eq!(did_doc.authentication(), &[verif_method]);
		assert_eq!(did_doc.pub_keys(), &[pubkey]);
		assert_eq!(did_doc.service(), &[service]);
	}

	#[test]
	fn did_document_builder_with_pub_keys() {
		assert_eq!(
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
				.build(),
			DidDocument {
				context: GENERIC_DID_CTX,
				id: "did:example:123456789abcdefghi",
				pub_keys: vec![
					PublicKey {
						id: "did:example:123456789abcdefghi#keys-1",
						key_type: PublicKeyType::Rsa,
						controller: "did:example:123456789abcdefghi",
						encoded_key: PublicKeyEncoded::Pem(
							"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
						),
					},
					PublicKey {
						id: "did:example:123456789abcdefghi#keys-2",
						key_type: PublicKeyType::Ed25519,
						controller: "did:example:pqrstuvwxyz0987654321",
						encoded_key: PublicKeyEncoded::Base58(
							"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
						),
					},
					PublicKey {
						id: "did:example:123456789abcdefghi#keys-3",
						key_type: PublicKeyType::EcdsaSecp256k1,
						controller: "did:example:123456789abcdefghi",
						encoded_key: PublicKeyEncoded::Hex(
							"02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
						),
					}
				],
				..Default::default()
			}
		)
	}

	#[test]
	fn did_document_builder_with_auth_reference_verif_method() {
		assert_eq!(
			DidDocumentBuilder::new("did:example:123456789abcdefghi")
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
				.build(),
			DidDocument {
				context: GENERIC_DID_CTX,
				id: "did:example:123456789abcdefghi",
				authentication: vec![VerificationMethod::Reference(
					"did:example:123456789abcdefghi#keys-1"
				)],
				pub_keys: vec![PublicKey {
					id: "did:example:123456789abcdefghi#keys-1",
					key_type: PublicKeyType::Rsa,
					controller: "did:example:123456789abcdefghi",
					encoded_key: PublicKeyEncoded::Pem(
						"-----BEGIN PUBLIC KEY...END PUBLIC KEY-----\r\n"
					),
				}],
				..Default::default()
			}
		)
	}

	#[test]
	fn did_document_builder_with_auth_embedded_verif_method() {
		assert_eq!(
			DidDocumentBuilder::new("did:example:123456789abcdefghi")
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
				.build(),
			DidDocument {
				context: GENERIC_DID_CTX,
				id: "did:example:123456789abcdefghi",
				authentication: vec![VerificationMethod::Embedded(PublicKey {
					id: "did:example:123456789abcdefghi#keys-2",
					key_type: PublicKeyType::Ed25519,
					controller: "did:example:123456789abcdefghi",
					encoded_key: PublicKeyEncoded::Base58(
						"H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
					),
				})],
				..Default::default()
			}
		)
	}

	#[test]
	fn did_document_builder_with_service_uri_endpoint() {
		assert_eq!(
			DidDocumentBuilder::new("did:example:123456789abcdefghi")
				.with_services(vec![Service::new(
					"did:example:123456789abcdefghi#openid",
					"OpenIdConnectVersion1.0Service",
					ServiceEndpoint::Uri("https://openid.example.com/")
				)])
				.build(),
			DidDocument {
				context: GENERIC_DID_CTX,
				id: "did:example:123456789abcdefghi",
				service: vec![Service {
					id: "did:example:123456789abcdefghi#openid",
					svc_type: "OpenIdConnectVersion1.0Service",
					endpoint: ServiceEndpoint::Uri("https://openid.example.com/")
				}],
				..Default::default()
			}
		)
	}

	#[test]
	fn did_document_builder_with_created() {
		assert_eq!(
			DidDocumentBuilder::new("did:example:123456789abcdefghi")
				.created_on("2002-10-10T17:00:00Z")
				.build(),
			DidDocument {
				context: GENERIC_DID_CTX,
				id: "did:example:123456789abcdefghi",
				created: Some("2002-10-10T17:00:00Z"),
				..Default::default()
			}
		)
	}

	#[test]
	fn did_document_builder_with_updated() {
		assert_eq!(
			DidDocumentBuilder::new("did:example:123456789abcdefghi")
				.updated_on("2002-10-10T17:00:00Z")
				.build(),
			DidDocument {
				context: GENERIC_DID_CTX,
				id: "did:example:123456789abcdefghi",
				updated: Some("2002-10-10T17:00:00Z"),
				..Default::default()
			}
		)
	}
}
