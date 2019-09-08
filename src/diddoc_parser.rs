use crate::{
	did::Did,
	did_doc::{
		DidDocument, DidDocumentBuilder, PublicKey, PublicKeyBuilder, PublicKeyEncoded,
		PublicKeyType, Service, ServiceEndpoint, VerificationMethod, KEY_FORMATS,
	},
};
use json::JsonValue;
use regex::Regex;
use std::str::FromStr;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";
const CONTEXT_PROP: &str = "@context";
const SUBJECT_PROP: &str = "id";
const CREATED_PROP: &str = "created";
const UPDATED_PROP: &str = "updated";
const PUBKEYS_PROP: &str = "publicKey";
const AUTHN_PROP: &str = "authentication";
const SERVICE_PROP: &str = "service";
const SVCENDP_PROP: &str = "serviceEndpoint";

const KEYID_PROP: &str = "id";
const KEYTYPE_PROP: &str = "type";
const KEYCTRL_PROP: &str = "controller";

lazy_static! {
	static ref DATETIME_REGEX: Regex = Regex::new(
		// See https://www.w3.org/TR/xmlschema11-2/#dateTime
		r"(?x)
		-?([1-9][0-9]{3,}|0[0-9]{3})
		-(0[1-9]|1[0-2])
		-(0[1-9]|[12][0-9]|3[01])
		T(([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9](\.[0-9]+)?|(24:00:00(\.0+)?))
		Z
    "
	)
	.unwrap();
}

fn parse_did_context(json: &JsonValue) -> Result<&str, &str> {
	match json[CONTEXT_PROP].as_str() {
		Some(GENERIC_DID_CTX) => Ok(GENERIC_DID_CTX),
		Some(_) => Err("invalid DID context"),
		None => Err("missing DID context"),
	}
}

fn parse_did_subject(json: &JsonValue) -> Result<&str, &str> {
	match json[SUBJECT_PROP].as_str() {
		Some(sub) => {
			if Did::is_valid(sub) {
				Ok(sub)
			} else {
				Err("invalid DID subject")
			}
		}
		None => Err("missing DID subject"),
	}
}

fn parse_did_created(json: &JsonValue) -> Result<Option<&str>, &str> {
	match json[CREATED_PROP].as_str() {
		Some(created) if DATETIME_REGEX.is_match(created) => Ok(Some(created)),
		Some(_) => Err("invalid created timestamp"),
		None => Ok(None),
	}
}

fn parse_did_updated(json: &JsonValue) -> Result<Option<&str>, &str> {
	match json[UPDATED_PROP].as_str() {
		Some(created) if DATETIME_REGEX.is_match(created) => Ok(Some(created)),
		Some(_) => Err("invalid updated timestamp"),
		None => Ok(None),
	}
}

fn parse_did_pubkey_list<'a>(json: &'a JsonValue) -> Result<Vec<PublicKey>, &'a str> {
	let mut keys: Vec<PublicKey> = vec![];
	for i in 0..json[PUBKEYS_PROP].len() {
		let key = &json[PUBKEYS_PROP][i];
		if key.is_null() {
			break;
		}
		let pubkey = parse_did_pubkey(key, &keys)?;
		keys.push(pubkey);
	}
	Ok(keys)
}

fn parse_did_pubkey<'a>(key: &'a JsonValue, keys: &[PublicKey]) -> Result<PublicKey<'a>, &'a str> {
	let key_id = parse_did_pubkey_id(key)?;
	if keys.iter().any(|k| k.id() == key_id) {
		// return Err(format!("duplicate DID public key id '{}'", key_id).as_str());
		return Err("duplicate DID public key id");
	}

	let key_type = parse_did_pubkey_type(key)?;
	let key_ctrl = parse_did_pubkey_ctrl(key)?;
	let key_format = parse_did_pubkey_format(key)?;
	let key_encoded = parse_did_pubkey_encoded(key, key_format)?;

	let key = PublicKeyBuilder::new(key_id, key_type, key_ctrl)
		.with_encoded_key(key_encoded)
		.build();

	Ok(key)
}

fn parse_did_pubkey_id(key: &JsonValue) -> Result<&str, &str> {
	match key[KEYID_PROP].as_str() {
		Some(id) => {
			if Did::is_valid(&id) {
				Ok(id)
			} else {
				Err("invalid DID public key id")
			}
		}
		None => Err("missing DID public key id"),
	}
}

fn parse_did_pubkey_type(key: &JsonValue) -> Result<PublicKeyType, &str> {
	match key[KEYTYPE_PROP].as_str() {
		Some(r#type) => match PublicKeyType::from_str(&r#type) {
			Ok(key_type) => Ok(key_type),
			Err(_) => Err("invalid DID public key type"),
		},
		None => Err("missing DID public key type"),
	}
}

fn parse_did_pubkey_ctrl(key: &JsonValue) -> Result<&str, &str> {
	match key[KEYCTRL_PROP].as_str() {
		Some(ctrl) => Ok(ctrl),
		None => Err("missing DID public key controller"),
	}
}

fn parse_did_pubkey_format(key: &JsonValue) -> Result<&str, &str> {
	match KEY_FORMATS.iter().find(|f| key.has_key(f)) {
		Some(&kf) => Ok(kf),
		None => Err("missing DID public key property"),
	}
}

fn parse_did_pubkey_encoded<'a>(
	key: &'a JsonValue,
	key_format: &'a str,
) -> Result<PublicKeyEncoded<'a>, &'a str> {
	match key[key_format].as_str() {
		Some(key_enc) => match PublicKeyEncoded::from((key_format, key_enc)) {
			PublicKeyEncoded::Unsupported => Err("unknown DID public key format"),
			supported => Ok(supported),
		},
		None => Err("missing DID public key controller"),
	}
}

fn parse_did_auth_list<'a>(
	json: &'a JsonValue,
	pub_keys: &[PublicKey],
) -> Result<Vec<VerificationMethod<'a>>, &'a str> {
	let mut verif_methods: Vec<VerificationMethod> = vec![];
	for i in 0..json[AUTHN_PROP].len() {
		let vm = &json[AUTHN_PROP][i];
		if vm.is_null() {
			break;
		}

		let verif_method = parse_auth_verif_method(vm, pub_keys)?;
		verif_methods.push(verif_method);
	}
	Ok(verif_methods)
}

fn parse_auth_verif_method<'a>(
	json: &'a JsonValue,
	pub_keys: &[PublicKey],
) -> Result<VerificationMethod<'a>, &'a str> {
	if json.is_string() {
		let did = match json.as_str() {
			Some(did) if Did::is_valid(did) => Ok(did),
			Some(_) => Err("invalid reference verification method"),
			None => Err("invalid reference verification method"),
		}?;

		if !pub_keys.iter().any(|k| k.id() == did) {
			return Err("unknown reference verification method");
		}

		Ok(VerificationMethod::Reference(did))
	} else if json.is_object() {
		let key = parse_did_pubkey(json, Vec::<PublicKey>::new().as_slice())?;

		if pub_keys.iter().any(|k| k.id() == key.id()) {
			return Err("duplicate public key id from embedded verification method");
		}
		Ok(VerificationMethod::Embedded(key))
	} else {
		Err("invalid embedded verification method")
	}
}

fn parse_did_services<'a>(json: &'a JsonValue) -> Result<Vec<Service<'a>>, &'a str> {
	let mut services: Vec<Service> = vec![];
	for i in 0..json[SERVICE_PROP].len() {
		let svc = &json[SERVICE_PROP][i];
		if svc.is_null() {
			break;
		}

		let service = parse_did_svc_endpoint(svc)?;
		services.push(service);
	}
	Ok(services)
}

fn parse_did_svc_endpoint(json: &JsonValue) -> Result<Service<'_>, &str> {
	let svc_id = match json[KEYID_PROP].as_str() {
		Some(id) => {
			if Did::is_valid(&id) {
				Ok(id)
			} else {
				Err("invalid service endpoint id")
			}
		}
		None => Err("missing service endpoint id"),
	}?;

	let svc_type = match json[KEYTYPE_PROP].as_str() {
		Some(svc_type) => Ok(svc_type),
		None => Err("missing service endpoint type"),
	}?;

	let svc_endpoint = if json[SVCENDP_PROP].is_string() {
		match json[SVCENDP_PROP].as_str() {
			Some(uri) => Ok(ServiceEndpoint::Uri(uri)),
			None => Err("invalid service endpoint URI"),
		}
	} else if json.is_object() {
		Err("invalid service endpoint JSON-LD object : unimplemented")
	} else {
		Err("invalid service endpoint : unknown format")
	}?;

	Ok(Service::new(svc_id, svc_type, svc_endpoint))
}

pub fn parse_did_doc(json: &JsonValue) -> Result<DidDocument<'_>, &str> {
	let _ctx = parse_did_context(json)?; //TODO: handle additional contexts beyond generic DID context
	let sub = parse_did_subject(json)?;
	let created = parse_did_created(json)?;
	let updated = parse_did_updated(json)?;
	let keys = parse_did_pubkey_list(json)?;
	let auth = parse_did_auth_list(json, &keys[..])?;
	let services = parse_did_services(json)?;

	let mut did_doc = DidDocumentBuilder::new(sub)
		.with_authentication(auth)
		.with_pubkeys(keys)
		.with_services(services);
	if let Some(created) = created {
		did_doc = did_doc.created_on(created);
	}
	if let Some(updated) = updated {
		did_doc = did_doc.updated_on(updated);
	}
	Ok(did_doc.build())
}
