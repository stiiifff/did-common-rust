use std::fmt;

use crate::did_parser;

#[derive(Debug, PartialEq)]
pub struct Did<'a> {
    pub method_name: &'a str,
    pub method_specific_id: &'a str,
    pub params: Option<Vec<DidParam<'a>>>,
    pub fragment: Option<&'a str>,
}

impl<'a> Did<'a> {
    pub fn new(method_name: &'a str, method_specific_id: &'a str) -> Did<'a> {
        Did {
            method_name,
            method_specific_id,
            params: None,
            fragment: None,
        }
    }

    pub fn with_params(
        method_name: &'a str,
        method_specific_id: &'a str,
        params: Vec<(&'a str, Option<&'a str>)>,
    ) -> Did<'a> {
        Did {
            method_name,
            method_specific_id,
            params: Some(
                params
                    .iter()
                    .map(|p| DidParam {
                        name: p.0,
                        value: p.1,
                    })
                    .collect(),
            ),
            fragment: None,
        }
    }

    pub fn is_valid(did_string: &str) -> bool {
        did_parser::validate_did(did_string)
    }

    pub fn parse(did_string: &'a str) -> Result<Self, &'a str> {
        match did_parser::parse_did(did_string) {
            Ok((_, did)) => Ok(did),
            Err(_) => Err("Failed to parse did."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DidParam<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

impl<'a> DidParam<'a> {
    pub fn new(name: &'a str, value: Option<&'a str>) -> DidParam<'a> {
        DidParam { name, value }
    }
}

impl fmt::Display for DidParam<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.name,
            match self.value {
                Some(val) => format!("={}", val),
                None => String::new(),
            }
        )
    }
}

impl fmt::Display for Did<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}{}",
            did_parser::DID_SCHEME,
            self.method_name,
            self.method_specific_id,
            match &self.params {
                Some(params) => format!(
                    ";{}",
                    params
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(";")
                ),
                None => String::new(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Did, DidParam};

    #[test]
    fn DidParam_impl_display_trait() {
        assert_eq!(format!("{}", DidParam::new("service", None)), "service");
        assert_eq!(
            format!("{}", DidParam::new("service", Some("agent"))),
            "service=agent"
        );
    }

    #[test]
    fn did_impl_display_trait() {
        assert_eq!(format!("{}", Did::new("example", "")), "did:example:");
        assert_eq!(
            format!("{}", Did::new("example", "1234")),
            "did:example:1234"
        );
        assert_eq!(
            format!(
                "{}",
                Did::with_params("example", "1234", vec![("service", Some("agent"))])
            ),
            "did:example:1234;service=agent"
        );
        assert_eq!(
            format!(
                "{}",
                Did::with_params(
                    "example",
                    "1234",
                    vec![("service", Some("agent")), ("example:foo:bar", Some("baz"))]
                )
            ),
            "did:example:1234;service=agent;example:foo:bar=baz"
        );
    }
}
