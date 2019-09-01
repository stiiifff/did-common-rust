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
    pub fn method_name(&self) -> &'a str {
        self.method_name
    }

    pub fn method_specific_id(&self) -> &'a str {
        self.method_specific_id
    }

    pub fn params(&self) -> Option<&[DidParam<'a>]> {
        match &self.params {
            Some(params) => Some(&params[..]),
            None => None
        }
    }

    pub fn fragment(&self) -> Option<&'a str> {
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
            "{}:{}:{}{}{}",
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
            },
            match &self.fragment {
                Some(fragment) => format!("#{}", fragment),
                None => String::new()
            },
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
    
    pub fn with_params<T: 'a, I: 'a>(&'a mut self, params: T) -> &'a mut Self 
        where T: IntoIterator<Item=I>,
              I: Into<DidParam<'a>>
    {
        self.params = Some(
            params
                .into_iter()
                .map(|p| p.into())
                .collect()
        );
        self
    }

    pub fn with_fragment(&'a mut self, fragment: &'a str) -> &'a mut Self {
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

type ParamTuple<'a> = (&'a str, &'a str);
type ParamOptionTuple<'a> = (&'a str, Option<&'a str>);

impl<'a> From<ParamOptionTuple<'a>> for DidParam<'a> {
    fn from(param: ParamOptionTuple<'a>) -> Self {
        DidParam::new(param.0, param.1)
    }
}

impl<'a> From<ParamTuple<'a>> for DidParam<'a> {
    fn from(param: ParamTuple<'a>) -> Self {
        DidParam::new(param.0, Some(param.1))
    }
}

//TODO: there is probably way than having to implement
//a From trait variant with &ParamOptionTuple (maybe using AsRef trait ?)
impl<'a> From<&ParamOptionTuple<'a>> for DidParam<'a> {
    fn from(param: &ParamOptionTuple<'a>) -> Self {
        DidParam::new(param.0, param.1)
    }
}

//TODO: there is probably way than having to implement
//a From trait variant with &ParamTuple (maybe using AsRef trait ?)
impl<'a> From<&ParamTuple<'a>> for DidParam<'a> {
    fn from(param: &ParamTuple<'a>) -> Self {
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
    fn did_property_accessors() {
        let did = Did {
            method_name: "example",
            method_specific_id: "1234",
            fragment: Some("keys-1"),
            params: Some(vec![DidParam{name:"example", value:None}])
        };
        assert_eq!(did.method_name(), "example");
        assert_eq!(did.method_specific_id(), "1234");
        assert_eq!(did.fragment(), Some("keys-1"));
        assert_eq!(did.params(), Some(&[DidParam{name:"example", value:None}][..]));
    }

    #[test]
    fn did_builder_for_simple_did() {
        assert_eq!(
            DidBuilder::new("example", "1234").build(),
            Did { method_name: "example", method_specific_id: "1234", fragment: None, params: None }
        )
    }

    #[test]
    fn did_builder_for_did_with_params() {
        assert_eq!(
            DidBuilder::new("example", "1234")
                .with_params(&[
                    ("service", None),
                    ("example:foo:bar",Some("baz"))
                ])
                .build(),
            Did {
                method_name: "example",
                method_specific_id: "1234",
                fragment: None,
                params: Some(vec![
                    DidParam {name:"service", value:None},
                    DidParam {name:"example:foo:bar", value:Some("baz")}
                ])
            }
        )
    }

    #[test]
    fn did_builder_for_complex_did() {
        assert_eq!(
            DidBuilder::new("example", "1234")
                .with_params(&[
                    ("service", None),
                    ("example:foo:bar",Some("baz"))
                ])
                .with_fragment("keys-1")
                .build(),
            Did {
                method_name: "example",
                method_specific_id: "1234",
                fragment: Some("keys-1"),
                params: Some(vec![
                    DidParam {name:"service", value:None},
                    DidParam {name:"example:foo:bar", value:Some("baz")}
                ])
            }
        )
    }

    #[test]
    fn did_impl_display_trait() {
        assert_eq!(format!("{}", 
            Did {
                method_name:"example",
                method_specific_id:"",
                fragment: None,
                params: None
            }), "did:example:");
        
        assert_eq!(
            format!("{}", 
                Did {
                    method_name:"example",
                    method_specific_id:"1234",
                    fragment: None,
                    params: None
            }),
            "did:example:1234"
        );

        assert_eq!(
            format!("{}", 
                Did {
                    method_name:"example",
                    method_specific_id:"1234",
                    fragment: Some("keys-1"),
                    params: None
            }),
            "did:example:1234#keys-1"
        );

        assert_eq!(
            format!("{}", 
                Did {
                    method_name:"example",
                    method_specific_id:"1234",
                    fragment: None,
                    params: Some(vec![
                        DidParam {name:"service", value:Some("agent")},
                        DidParam {name:"example:foo:bar", value:Some("baz")}
                ])
            }),
            "did:example:1234;service=agent;example:foo:bar=baz"
        );

        assert_eq!(
            format!("{}", 
                Did {
                    method_name:"example",
                    method_specific_id:"1234",
                    fragment: Some("keys-1"),
                    params: Some(vec![
                        DidParam {name:"service", value:Some("agent")},
                        DidParam {name:"example:foo:bar", value:Some("baz")}
                ])
            }),
            "did:example:1234;service=agent;example:foo:bar=baz#keys-1"
        );
    }

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
