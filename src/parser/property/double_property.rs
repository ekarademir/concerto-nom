use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::into,
    error::context,
    multi::fold_many_m_n,
    sequence::{preceded, tuple},
    Parser,
};

use crate::parser::{
    common::{keywords, numeric::double_value},
    property::internal::{primitive_property, ranged_parser, PrimitiveType, Ranged},
    CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct DoubleProperty {
    pub name: String,
    pub default_value: Option<f64>,
    pub domain_validator: Option<DoubleDomainValidator>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoubleDomainValidator {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

impl From<Ranged<f64>> for DoubleDomainValidator {
    fn from(value: Ranged<f64>) -> Self {
        Self {
            lower: value.start,
            upper: value.end,
        }
    }
}
enum DoubleMetaProperty {
    Default(f64),
    Domain(DoubleDomainValidator),
}

/// Parses a primitive DoubleProperty with its default meta properties.
/// If a meta property is defined twice, second one will overwrite the first.
/// Meta property parser will only run two times.
pub fn double_property<'a>(input: &'a str) -> CResult<&'a str, DoubleProperty> {
    let domain = context(
        "DoubleDomainValidator",
        preceded(space1, double_domain_validator),
    )
    .map(|x| DoubleMetaProperty::Domain(x));
    let default = preceded(space1, double_default_value).map(|x| DoubleMetaProperty::Default(x));

    let property_meta = context("PropertyMeta", alt((domain, default)));

    context(
        "DoubleProperty",
        primitive_property(PrimitiveType::DoublePropertyType)
            .and(fold_many_m_n(
                0,
                2,
                property_meta,
                Vec::new,
                |mut acc: Vec<_>, meta_prop| {
                    acc.push(meta_prop);
                    acc
                },
            ))
            .map(|(property_name, meta_props)| {
                let mut prop = DoubleProperty {
                    name: property_name.to_string(),
                    default_value: None,
                    domain_validator: None,
                };

                for meta_prop in meta_props {
                    use DoubleMetaProperty::*;
                    match meta_prop {
                        Default(x) => prop.default_value = Some(x),
                        Domain(x) => prop.domain_validator = Some(x),
                    }
                }

                prop
            }),
    )(input)
}

pub fn double_default_value<'a>(input: &'a str) -> CResult<&'a str, f64> {
    into(context(
        "DoubleDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            double_value,
        ),
    ))(input)
}

pub fn double_domain_validator<'a>(input: &'a str) -> CResult<&'a str, DoubleDomainValidator> {
    match ranged_parser(input, keywords::range, double_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, ranged.into())),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_double_property() {
        assert_eq!(
            super::double_property("o Double foo"),
            Ok((
                "",
                super::DoubleProperty {
                    name: String::from("foo"),
                    default_value: None,
                    domain_validator: None
                }
            )),
            "Should parse double with no meta properties"
        );

        assert_eq!(
            super::double_property("o Double baz default=42.0"),
            Ok((
                "",
                super::DoubleProperty {
                    name: String::from("baz"),
                    default_value: Some(42.0),
                    domain_validator: None
                }
            )),
            "Should parse double with default value only"
        );

        assert_eq!(
            super::double_property("o Double baz    range   = [ 0.0 , 10.0  ]"),
            Ok((
                "",
                super::DoubleProperty {
                    name: String::from("baz"),
                    default_value: None,
                    domain_validator: Some(super::DoubleDomainValidator {
                        lower: Some(0.0),
                        upper: Some(10.0)
                    })
                }
            )),
            "Should parse double with range only"
        );

        assert_eq!(
            super::double_property("o Double baz \tdefault  =   -42.0e3    range=[,100.4]"),
            Ok((
                "",
                super::DoubleProperty {
                    name: String::from("baz"),
                    default_value: Some(-42.0e3),
                    domain_validator: Some(super::DoubleDomainValidator {
                        lower: None,
                        upper: Some(100.4)
                    })
                }
            )),
            "Should parse double with both default and range"
        );

        assert_eq!(
            super::double_property("o Double baz \trange=[,  100.0 ] \tdefault  =   42.5e-3"),
            Ok((
                "",
                super::DoubleProperty {
                    name: String::from("baz"),
                    default_value: Some(42.5e-3),
                    domain_validator: Some(super::DoubleDomainValidator {
                        lower: None,
                        upper: Some(100.0)
                    })
                }
            )),
            "Should parse double with both default and range in a different order"
        );
    }
}
