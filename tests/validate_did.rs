use did_common::{Did};

#[test]
fn validate_generic_did() {
    assert_eq!(Did::is_valid("did:example:1234"), true);
}

#[test]
fn validate_generic_did_with_fragment() {
    assert_eq!(
        Did::is_valid("did:example:123456789abcdefghi#keys-1"),
        true
    )
}

#[test]
fn validate_did_with_missing_method_name() {
    assert_eq!(Did::is_valid("did::123456"), false);
}

#[test]
fn validate_did_with_invalid_method_name() {
    assert_eq!(Did::is_valid("did:EXAMPLE:123456"), false);
}

#[test]
fn validate_ethr_did() {
   assert_eq!(Did::is_valid("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"), true);
}

#[test]
fn validate_did_with_generic_param() {
    assert_eq!(Did::is_valid("did:example:1234"), true);
}

#[test]
fn validate_did_with_multiple_params() {
    assert_eq!(Did::is_valid("did:example:1234;service=agent;example:foo:bar=baz"), true);
}

#[test]
fn parse_did_with_multiple_params_and_fragment() {
    assert_eq!(
        Did::is_valid("did:example:1234;service=agent;example:foo:bar=baz#keys-1"),
        true
    );
}
