use crate::lib::std::str::FromStr;
use crate::lib::std::vec::Vec;

use crate::{
	did::Did,
	did_doc::{
		DidDocument, DidDocumentBuilder, PublicKey, PublicKeyBuilder, PublicKeyEncoded,
		PublicKeyType, Service, ServiceEndpoint, VerificationMethod, KEY_FORMATS,
	},
};

use json::JsonValue;
#[cfg(feature = "std")]
use regex::Regex;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";
const CONTEXT_PROP: &str = "@context";
const SUBJECT_PROP: &str = "id";
const CREATED_PROP: &str = "created";
const UPDATED_PROP: &str = "updated";
const PUBKEYS_PROP: &str = "publicKey";
const AUTHN_PROP: &str = "authentication";
const SERVICE_PROP: &str = "service";
const SVCENDP_PROP: &str = "serviceEndpoint";

const ID_PROP: &str = "id";
const TYPE_PROP: &str = "type";
const CTRL_PROP: &str = "controller";

#[cfg(feature = "std")]
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

#[inline]
fn parse_str<'a>(json: &'a JsonValue, prop: &str, err: &'a str) -> Result<&'a str, &'a str> {
	json[prop].as_str().ok_or(err)
}

#[inline]
fn parse_str_then<'a, F: FnOnce(&'a str) -> Result<&'a str, &'a str>>(
	json: &'a JsonValue,
	prop: &str,
	err: &'a str,
	op: F,
) -> Result<&'a str, &'a str> {
	parse_str(json, prop, err).and_then(op)
}

fn parse_did_context(json: &JsonValue) -> Result<&str, &str> {
	parse_str_then(json, CONTEXT_PROP, "missing DID context", |ctx| {
		if ctx == GENERIC_DID_CTX {
			Ok(GENERIC_DID_CTX)
		} else {
			Err("invalid DID context")
		}
	})
}

fn parse_did_subject(json: &JsonValue) -> Result<&str, &str> {
	parse_str_then(json, SUBJECT_PROP, "missing DID subject", |sub| {
		if Did::is_valid(sub) {
			Ok(sub)
		} else {
			Err("invalid DID subject")
		}
	})
}

#[cfg(feature = "std")]
fn validate_datetime(input: &str) -> bool {
	DATETIME_REGEX.is_match(input)
}

#[cfg(not(feature = "std"))]
fn validate_datetime(_input: &str) -> bool {
	true
}

fn parse_did_created(json: &JsonValue) -> Result<Option<&str>, &str> {
	match json[CREATED_PROP].as_str() {
		Some(created) if validate_datetime(created) => Ok(Some(created)),
		Some(_) => Err("invalid created timestamp"),
		None => Ok(None),
	}
}

fn parse_did_updated(json: &JsonValue) -> Result<Option<&str>, &str> {
	match json[UPDATED_PROP].as_str() {
		#[cfg(feature = "std")]
		Some(created) if validate_datetime(created) => Ok(Some(created)),
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
	parse_str_then(key, ID_PROP, "missing DID public key id", |id| {
		if Did::is_valid(&id) {
			Ok(id)
		} else {
			Err("invalid DID public key id")
		}
	})
}

fn parse_did_pubkey_type(key: &JsonValue) -> Result<PublicKeyType, &str> {
	parse_str(key, TYPE_PROP, "missing DID public key type").and_then(|r#type| {
		match PublicKeyType::from_str(&r#type) {
			Ok(key_type) => Ok(key_type),
			Err(_) => Err("invalid DID public key type"),
		}
	})
}

fn parse_did_pubkey_ctrl(key: &JsonValue) -> Result<&str, &str> {
	parse_str(key, CTRL_PROP, "missing DID public key controller")
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
	parse_str(key, key_format, "missing DID public key controller").and_then(|key_enc| {
		match PublicKeyEncoded::from((key_format, key_enc)) {
			PublicKeyEncoded::Unsupported => Err("unknown DID public key format"),
			supported => Ok(supported),
		}
	})
}

fn parse_did_auth_list<'a>(
	json: &'a JsonValue,
	pub_keys: &[PublicKey],
) -> Result<Vec<VerificationMethod<'a>>, &'a str> {
	json[AUTHN_PROP]
		.members()
		.map(|vm| parse_auth_verif_method(vm, pub_keys))
		.collect()
}

fn parse_auth_verif_method<'a>(
	json: &'a JsonValue,
	pub_keys: &[PublicKey],
) -> Result<VerificationMethod<'a>, &'a str> {
	if json.is_string() {
		let did = parse_auth_verif_method_ref(json)?;
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

fn parse_auth_verif_method_ref(json: &JsonValue) -> Result<&str, &str> {
	json.as_str()
		.ok_or("invalid reference verification method")
		.and_then(|did| {
			if Did::is_valid(did) {
				Ok(did)
			} else {
				Err("invalid reference verification method")
			}
		})
}

fn parse_did_service_list<'a>(json: &'a JsonValue) -> Result<Vec<Service<'a>>, &'a str> {
	json[SERVICE_PROP]
		.members()
		.map(parse_did_svc_endpoint)
		.collect()
}

fn parse_did_svc_endpoint(json: &JsonValue) -> Result<Service<'_>, &str> {
	let svc_id = parse_did_svc_endpoint_id(json)?;
	let svc_type = parse_did_svc_endpoint_type(json)?;
	let svc_endpoint = parse_did_svc_endpoint_value(json)?;
	Ok(Service::new(svc_id, svc_type, svc_endpoint))
}

fn parse_did_svc_endpoint_id(key: &JsonValue) -> Result<&str, &str> {
	parse_str_then(key, ID_PROP, "missing service endpoint id", |id| {
		if Did::is_valid(&id) {
			Ok(id)
		} else {
			Err("invalid service endpoint id")
		}
	})
}

fn parse_did_svc_endpoint_type(json: &JsonValue) -> Result<&str, &str> {
	parse_str(json, TYPE_PROP, "missing service endpoint type")
}

fn parse_did_svc_endpoint_value(json: &JsonValue) -> Result<ServiceEndpoint, &str> {
	if json[SVCENDP_PROP].is_string() {
		parse_str(json, SVCENDP_PROP, "invalid service endpoint URI")
			.and_then(|uri| Ok(ServiceEndpoint::Uri(uri)))
	} else if json.is_object() {
		Err("invalid service endpoint JSON-LD object : unimplemented")
	} else {
		Err("invalid service endpoint : unknown format")
	}
}

pub fn parse_did_doc(json: &JsonValue) -> Result<DidDocument<'_>, &str> {
	let _ctx = parse_did_context(json)?; //TODO: handle additional contexts beyond generic DID context
	let sub = parse_did_subject(json)?;
	let created = parse_did_created(json)?;
	let updated = parse_did_updated(json)?;
	let keys = parse_did_pubkey_list(json)?;
	let auth = parse_did_auth_list(json, &keys[..])?;
	let services = parse_did_service_list(json)?; //TODO: validate URI, handle embedded service object + extra props

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
