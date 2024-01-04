use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::into,
    error::context,
    sequence::{preceded, tuple},
    Parser,
};

use crate::parser::{
    common::{boolean_value, keywords},
    property::internal::{primitive_property, PrimitiveType},
    CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanProperty {
    pub name: String,
    pub default_value: Option<bool>,
}

pub fn boolean_property<'a>(input: &'a str) -> CResult<&'a str, BooleanProperty> {
    let boolean_with_default = context(
        "BooleanWithDefault",
        primitive_property(PrimitiveType::BooleanPropertyType)
            .and(preceded(space1, boolean_default_value))
            .map(|(property_name, default_value)| BooleanProperty {
                default_value: Some(default_value),
                name: property_name.to_string(),
            }),
    );
    let boolean_without_default = context(
        "BooleanWithoutDefault",
        primitive_property(PrimitiveType::BooleanPropertyType).map(|property_name| {
            BooleanProperty {
                default_value: None,
                name: property_name.to_string(),
            }
        }),
    );

    context(
        "BooleanProperty",
        alt((boolean_with_default, boolean_without_default)),
    )(input)
}

pub fn boolean_default_value<'a>(input: &'a str) -> CResult<&'a str, bool> {
    into(context(
        "BooleanDefaultValue",
        preceded(
            tuple((keywords::default, space0, char('='), space0)),
            boolean_value,
        ),
    ))(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_boolean_property() {
        assert_eq!(
            super::boolean_property("o Boolean foo"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("foo"),
                    default_value: None,
                }
            )),
            "Should parse boolean with no meta properties"
        );

        assert_eq!(
            super::boolean_property("o Boolean baz default=false"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("baz"),
                    default_value: Some(false),
                }
            )),
            "Should parse boolean with false default value"
        );

        assert_eq!(
            super::boolean_property("o Boolean baz default=true"),
            Ok((
                "",
                super::BooleanProperty {
                    name: String::from("baz"),
                    default_value: Some(true),
                }
            )),
            "Should parse boolean with true default value"
        );

        assert_eq!(
            super::boolean_property("o Boolean baz default=42"),
            Ok((
                " default=42",
                super::BooleanProperty {
                    name: String::from("baz"),
                    default_value: None,
                }
            )),
            "Should not parse boolean with wrong default value"
        );
    }
}
