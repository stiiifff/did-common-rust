use super::{DID, DIDDocument};
use json::parse;

pub const GENERIC_DID_CTX: &str = "https://www.w3.org/2019/did/v1";

pub fn parse_diddoc(input: &str) -> Result<DIDDocument, &str> {
    let json = parse(input).map_err(|_| "Failed to parse did document.")?;
    // let ctx = json["context"].as_str().unwrap();
    let id = json["id"].as_str().unwrap();
    let did = DID::parse(id).unwrap();
    Ok(DIDDocument::new(did))
    
    // match (json["context"].as_str(), json["id"].as_str()) {
    //     (Some(GENERIC_DID_CTX), Some(id)) => {
    //         let did = DID::parse(id).unwrap(); //.map(|did| DIDDocument::new(did)),
    //         Ok(DIDDocument::<'a>::new(did))
    //     },
    //     (_,_) => Err("Failed to parse DID document.")
    // }
}
