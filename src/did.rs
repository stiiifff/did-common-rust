use std::fmt;
use std::convert::From;

use crate::did_parser;

#[derive(Debug, PartialEq)]
pub struct Did<'a> {
    method_name: &'a str,
    method_specific_id: &'a str,
    params: Option<Vec<DidParam<'a>>>,
    fragment: Option<&'a str>,
}

impl<'a> Did<'a> {
    pub fn method_name(self) -> &'a str {
        self.method_name
    }

    pub fn method_specific_id(self) -> &'a str {
        self.method_specific_id
    }

    pub fn params(self) -> Option<Vec<DidParam<'a>>> {
        self.params
    }

    pub fn fragment(self) -> Option<&'a str> {
        self.fragment
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

#[derive(Debug, PartialEq)]
pub struct DidBuilder<'a> {
    method_name: &'a str,
    method_specific_id: &'a str,
    params: Option<Vec<DidParam<'a>>>,
    fragment: Option<&'a str>,
}

impl<'a> DidBuilder<'a> {
    pub fn new(method_name: &'a str, method_specific_id: &'a str) -> Self {
        DidBuilder {
            method_name,
            method_specific_id,
            params: None,
            fragment: None,
        }
    }

    pub fn with_params(&mut self, params: impl Iterator<Item=(&'a str, Option<&'a str>)>) -> &mut Self {
        self.params = Some(
            params.map(|p| p.into()).collect()
        );
        self
    }

    pub fn with_fragment(&mut self, fragment: &'a str) -> &mut Self {
        self.fragment = Some(fragment);
        self
    }

    pub fn build(self) -> Did<'a> {
        Did {
            method_name: self.method_name,
            method_specific_id: self.method_specific_id,
            params: self.params,
            fragment: self.fragment,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DidParam<'a> {
    name: &'a str,
    value: Option<&'a str>,
}

impl<'a> DidParam<'a> {
    pub fn name(self) -> &'a str {
        self.name
    }

    pub fn value(self) -> Option<&'a str> {
        self.value
    }

    pub fn new(name: &'a str, value: Option<&'a str>) -> DidParam<'a> {
        DidParam { name, value }
    }
}

impl<'a> From<(&'a str, Option<&'a str>)> for DidParam<'a> {
    fn from(param: (&'a str, Option<&'a str>)) -> Self {
        DidParam::new(param.0, param.1)
    }
}

impl<'a> From<(&'a str, &'a str)> for DidParam<'a> {
    fn from(param: (&'a str, &'a str)) -> Self {
        DidParam::new(param.0, Some(param.1))
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

#[cfg(test)]
mod tests {
    use super::{Did, DidBuilder, DidParam};

    #[test]
    fn did_builder_for_simple_did() {
        assert_eq!(
            DidBuilder::new("example", "1234").build(),
            Did { method_name: "example", method_specific_id: "1234", fragment: None, params: None }
        )
    }

    // #[test]
    // fn DidBuilder_works_for_did_with_params() {
    //     assert_eq!(
    //         DidBuilder::new("example", "1234")
    //             .with_params([("service", Option::<&str>::None)])
    //             .build(),
    //         Did { method_name: "example", method_specific_id: "1234", fragment: None, params: None }
    //     )
    // }

    // #[test]
    // fn did_impl_display_trait() {
    //     assert_eq!(format!("{}", DidBuilder::new("example", "").build()), "did:example:");
    //     assert_eq!(
    //         format!("{}", DidBuilder::new("example", "1234").build()),
    //         "did:example:1234"
    //     );
    //     assert_eq!(
    //         format!(
    //             "{}",
    //             DidBuilder::new("example", "1234", vec![("service", Some("agent"))])
    //         ),
    //         "did:example:1234;service=agent"
    //     );
    //     assert_eq!(
    //         format!(
    //             "{}",
    //             Did::with_params(
    //                 "example",
    //                 "1234",
    //                 vec![("service", Some("agent")), ("example:foo:bar", Some("baz"))]
    //             )
    //         ),
    //         "did:example:1234;service=agent;example:foo:bar=baz"
    //     );
    // }

    #[test]
    fn did_param_ctor_without_value() {
        assert_eq!(
            DidParam::new("service", None),
            DidParam { name: "service", value: None }
        )
    }

    #[test]
    fn did_param_property_accessors() {
        assert_eq!(
            DidParam::new("service", None).name(),
            "service"
        );
        assert_eq!(
            DidParam::new("service", None).value(),
            None
        );
        assert_eq!(
            DidParam::new("service", Some("agent")).value(),
            Some("agent")
        );
    }

    #[test]
    fn did_param_ctor_with_value() {
        assert_eq!(
            DidParam::new("service", Some("agent")),
            DidParam { name: "service", value: Some("agent") }
        )
    }

    #[test]
    fn did_param_from_trait_without_value() {
        assert_eq!(
            DidParam::from(("service", None)),
            DidParam { name: "service", value: None }
        )
    }

    #[test]
    fn did_param_from_trait_with_mandatory_value() {
        assert_eq!(
            DidParam::from(("service", Some("agent"))),
            DidParam { name: "service", value: Some("agent") }
        )
    }

    #[test]
    fn did_param_from_trait_with_optional_value() {
        assert_eq!(
            DidParam::from(("service", "agent")),
            DidParam { name: "service", value: Some("agent") }
        )
    }

    #[test]
    fn did_param_display_trait_without_value() {
        assert_eq!(
            format!("{}", DidParam::new("service", None)),
            "service");
    }

    #[test]
    fn did_param_display_trait_with_value() {
        assert_eq!(
            format!("{}", DidParam::new("service", Some("agent"))),
            "service=agent"
        );
    }
}
