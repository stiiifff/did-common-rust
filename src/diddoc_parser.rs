use super::{DID, DIDDocument};
use json::parse;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";
const CONTEXT_PROP: &str = "@context";
const SUBJECT_PROP: &str = "id";

pub fn parse_did_doc<'a>(input: &'a str) -> Result<DIDDocument , &'a str> {
    let json = parse(input).map_err(|_| "Failed to parse did document.")?;
    println!("{:?}", json);

    let _ctx = match json[CONTEXT_PROP].as_str() {
        Some(GENERIC_DID_CTX) => Ok(GENERIC_DID_CTX),
        // Some(ctx) => Err(format!("unknown DID context '{}'", ctx.to_owned()).as_ref()),
        Some(_) => Err("invalid DID context"),
        None => Err("missing DID context")
    }?;

    let did = match json[SUBJECT_PROP].as_str() {
        Some(sub) => {
            match DID::parse(sub) {
                Ok(did) => Ok(did),
                // Err(err) => Err(format!("invalid DID subject: {}", err).as_str())
                Err(_) => Err("invalid DID subject")
            }
        },
        None => Err("missing DID subject")
    }?;

    Ok(
        DIDDocument::new("")
    )
}

#[cfg(test)]
mod tests {
    use super::{DIDDocument, DID};
    use super::parse_did_doc;

    #[test]
    fn parse_did_doc_with_missing_context() {
        assert_eq!(
            parse_did_doc(r#"
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
            parse_did_doc(r#"
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
            parse_did_doc(r#"
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
            parse_did_doc(r#"
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
            parse_did_doc(r#"
            {
                "@context": "https://www.w3.org/2019/did/v1",
                "id": "did:example:21tDAKCERh95uGgKbJNHYp"
            }
            "#),
            Ok(DIDDocument::new("did:example:21tDAKCERh95uGgKbJNHYp"))
        );
    }
}
