use nom::{
    character::complete::{char, space0, space1},
    combinator::{into, opt},
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::parser::{
    common::{datetime::datetime_value, keywords},
    property::internal::{primitive_property, PrimitiveType},
    CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct DateTimeProperty {
    pub name: String,
    pub default_value: Option<String>,
}

pub fn datetime_property<'a>(input: &'a str) -> CResult<&'a str, DateTimeProperty> {
    context(
        "DateTimeProperty",
        primitive_property(PrimitiveType::DateTimePropertyType)
            .and(opt(preceded(space1, datetime_default_value)))
            .map(|(property_name, default_value)| DateTimeProperty {
                default_value,
                name: property_name.to_string(),
            }),
    )(input)
}

pub fn datetime_default_value<'a>(input: &'a str) -> CResult<&'a str, String> {
    into(context(
        "DateTimeDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            datetime_value,
        ),
    ))(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_datetime_property() {
        assert_eq!(
            super::datetime_property("o DateTime foo"),
            Ok((
                "",
                super::DateTimeProperty {
                    name: String::from("foo"),
                    default_value: None,
                }
            )),
            "Should parse datetime with no meta properties"
        );

        assert_eq!(
            super::datetime_property("o DateTime baz default=2024-01-04T18:39:55+02:30"),
            Ok((
                "",
                super::DateTimeProperty {
                    name: String::from("baz"),
                    default_value: Some(String::from("2024-01-04T18:39:55+02:30")),
                }
            )),
            "Should parse datetime with default value"
        );

        assert_eq!(
            super::datetime_property("o DateTime baz default=42"),
            Ok((
                " default=42",
                super::DateTimeProperty {
                    name: String::from("baz"),
                    default_value: None,
                }
            )),
            "Should not parse datetime with wring default value"
        );
    }
}
