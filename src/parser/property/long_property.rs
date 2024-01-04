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
    common::{keywords, numeric::long_value},
    property::internal::{primitive_property, ranged_parser, PrimitiveType, Ranged},
    CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct LongProperty {
    pub name: String,
    pub default_value: Option<i64>,
    pub domain_validator: Option<LongDomainValidator>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LongDomainValidator {
    pub lower: Option<i64>,
    pub upper: Option<i64>,
}

impl From<Ranged<i64>> for LongDomainValidator {
    fn from(value: Ranged<i64>) -> Self {
        Self {
            lower: value.start,
            upper: value.end,
        }
    }
}
enum LongMetaProperty {
    Default(i64),
    Domain(LongDomainValidator),
}

/// Parses a primitive LongProperty with its default meta properties.
/// If a meta property is defined twice, second one will overwrite the first.
/// Meta property parser will only run two times.
pub fn long_property<'a>(input: &'a str) -> CResult<&'a str, LongProperty> {
    let domain = context(
        "LongDomainValidator",
        preceded(space1, long_domain_validator),
    )
    .map(|x| LongMetaProperty::Domain(x));
    let default = preceded(space1, long_default_value).map(|x| LongMetaProperty::Default(x));

    let property_meta = context("PropertyMeta", alt((domain, default)));

    context(
        "LongProperty",
        primitive_property(PrimitiveType::LongPropertyType)
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
                let mut prop = LongProperty {
                    name: property_name.to_string(),
                    default_value: None,
                    domain_validator: None,
                };

                for meta_prop in meta_props {
                    use LongMetaProperty::*;
                    match meta_prop {
                        Default(x) => prop.default_value = Some(x),
                        Domain(x) => prop.domain_validator = Some(x),
                    }
                }

                prop
            }),
    )(input)
}

pub fn long_default_value<'a>(input: &'a str) -> CResult<&'a str, i64> {
    into(context(
        "LongDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            long_value,
        ),
    ))(input)
}

pub fn long_domain_validator<'a>(input: &'a str) -> CResult<&'a str, LongDomainValidator> {
    match ranged_parser(input, keywords::range, long_value) {
        Err(e) => Err(e),
        Ok((remains, ranged)) => Ok((remains, ranged.into())),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_long_property() {
        assert_eq!(
            super::long_property("o Long foo"),
            Ok((
                "",
                super::LongProperty {
                    name: String::from("foo"),
                    default_value: None,
                    domain_validator: None
                }
            )),
            "Should parse long with no meta properties"
        );

        assert_eq!(
            super::long_property("o Long baz default=42"),
            Ok((
                "",
                super::LongProperty {
                    name: String::from("baz"),
                    default_value: Some(42),
                    domain_validator: None
                }
            )),
            "Should parse long with default value only"
        );

        assert_eq!(
            super::long_property("o Long baz    range   = [ 0 , 10  ]"),
            Ok((
                "",
                super::LongProperty {
                    name: String::from("baz"),
                    default_value: None,
                    domain_validator: Some(super::LongDomainValidator {
                        lower: Some(0),
                        upper: Some(10)
                    })
                }
            )),
            "Should parse long with range only"
        );

        assert_eq!(
            super::long_property("o Long baz \tdefault  =   -42    range=[,100]"),
            Ok((
                "",
                super::LongProperty {
                    name: String::from("baz"),
                    default_value: Some(-42),
                    domain_validator: Some(super::LongDomainValidator {
                        lower: None,
                        upper: Some(100)
                    })
                }
            )),
            "Should parse long with both default and range"
        );

        assert_eq!(
            super::long_property("o Long baz \trange=[,  100 ] \tdefault  =   42"),
            Ok((
                "",
                super::LongProperty {
                    name: String::from("baz"),
                    default_value: Some(42),
                    domain_validator: Some(super::LongDomainValidator {
                        lower: None,
                        upper: Some(100)
                    })
                }
            )),
            "Should parse long with both default and range in a different order"
        );
    }
}
