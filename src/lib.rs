// #![no_main]
#![no_std]

#[macro_use] extern crate lazy_static;

extern crate nom;
pub mod did_parser;

#[cfg(test)]
mod tests {
    use super::did_parser::{parse_did, DID};

    #[test]
    fn parse_generic_did() {
        assert_eq!(
            parse_did("did:example:1234"),
            Ok(("", DID::new("example", "1234")))
        );
    }

    #[test]
    fn parse_generic_id_with_empty_method_id() {
        assert_eq!(parse_did("did:example:"), Ok(("", DID::new("example", ""))));
    }

    #[test]
    fn parse_btcr_did_with_key() {
        assert_eq!(
            parse_did("did:btcr:xyv2-xzpq-q9wa-p7t#satoshi"),
            Ok(("#satoshi", DID::new("btcr", "xyv2-xzpq-q9wa-p7t")))
        )
    }

    #[test]
    fn parse_ethr_did() {
        assert_eq!(
            parse_did("did:ethr:0xf3beac30c498d9e26865f34fcaa57dbb935b0d74"),
            Ok((
                "",
                DID::new("ethr", "0xf3beac30c498d9e26865f34fcaa57dbb935b0d74")
            ))
        );
    }

    #[test]
    fn parse_sovrin_did() {
        assert_eq!(
            parse_did("did:sov:2wJPyULfLLnYTEFYzByfUR"),
            Ok(("", DID::new("sov", "2wJPyULfLLnYTEFYzByfUR")))
        );
    }

    #[test]
    fn parse_erc725_did() {
        assert_eq!(
            parse_did("did:erc725:ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E"),
            Ok((
                "",
                DID::new("erc725", "ropsten:2F2B37C890824242Cb9B0FE5614fA2221B79901E")
            ))
        )
    }

    #[test]
    fn parse_veres_one_did() {
        assert_eq!(
            parse_did("did:v1:uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74"),
            Ok((
                "",
                DID::new("v1", "uuid:804c6ac3-ce3b-46ce-b134-17175d5bee74")
            ))
        )
    }
}
