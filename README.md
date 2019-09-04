did-common
==========

[![Build Status](https://app.codeship.com/projects/db6e3400-a583-0137-4b80-166c57de4e20/status?branch=master)](https://app.codeship.com/projects/360660)

A rust library for parsing Decentralized Identifiers (DIDs) following
the [DID specification](https://w3c-ccg.github.io/did-spec/) by the W3C.

### Usage: DID

Add this to your `Cargo.toml`:

```toml
[dependencies]
did-common = "^0.1"
```

and this to your crate root (if you're using Rust 2015):

```rust
extern crate did_common;
```

Here is how to to parse or validate a simple DID string:
```rust
use did_common::did::Did;

let did = Did::parse("did:example:123456789abcdefghi#keys-1").unwrap();

if Did::is_valid("did:example:") {
  println!("DID is valid.");
}
// output: DID is valid.
```

You can also build a DID using a builder:
```rust
use did_common::did::DidBuilder;

let did =
  DidBuilder::new("example", "1234")
    .with_params(&[("service", Some("agent"))])
    .with_fragment("keys-1")
    .build();
println!("{}", did);
// output: did:example:1234;service=agent#keys-1
```

### Usage: DID Document

Here is how to parse a simple DID Document:
```rust
use did_common::did_doc::DidDocument;
use did_common::json_parse;

let did_doc = DidDocument::parse(
  &json_parse(
  r#"
    {
      "@context": "https://www.w3.org/2019/did/v1",
      "id": "did:example:123456789abcdefghi",
      "publicKey": [
        {
            "id": "did:example:123456789abcdefghi#keys-1",
            "type": "Secp256k1VerificationKey2018",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
        }
      ]
    }
  "#).unwrap()
);
println!("{}", did_doc.id());
// output: did:example:21tDAKCERh95uGgKbJNHYp
```

You can also build a DID Document using a builder:
```rust
use did_common::did_doc::DidDocumentBuilder;

let did_doc =
  DidDocumentBuilder::new("did:example:123456789abcdefghi")
    .with_pubkeys(vec![
        PublicKeyBuilder::new(
            "did:example:123456789abcdefghi#keys-1",
            PublicKeyType::EcdsaSecp256k1,
            "did:example:123456789abcdefghi"
        )
        .with_encoded_key(PublicKeyEncoded::Hex(
            "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
        ))
        .build()
    ])
    .build();

let key = did_doc.pub_keys().first().unwrap();
println!("{}", key.id());
// output: did:example:123456789abcdefghi#keys-1
```

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
