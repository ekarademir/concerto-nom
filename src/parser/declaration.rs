use nom::{
    branch::alt,
    character::complete::{char, line_ending, multispace0, space0, space1},
    combinator::into,
    error::context,
    multi::fold_many0,
    sequence::{delimited, tuple},
    Parser,
};

use crate::parser::{
    common::{keywords, token},
    property, CResult,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Property {
    Boolean(property::boolean_property::BooleanProperty),
    Integer(property::integer_property::IntegerProperty),
    Long(property::long_property::LongProperty),
    Double(property::double_property::DoubleProperty),
    DateTime(property::datetime_property::DateTimeProperty),
    String(property::string_property::StringProperty),
    Imported(property::Property),
}

impl From<property::boolean_property::BooleanProperty> for Property {
    fn from(value: property::boolean_property::BooleanProperty) -> Self {
        Self::Boolean(value)
    }
}

impl From<property::long_property::LongProperty> for Property {
    fn from(value: property::long_property::LongProperty) -> Self {
        Self::Long(value)
    }
}

impl From<property::integer_property::IntegerProperty> for Property {
    fn from(value: property::integer_property::IntegerProperty) -> Self {
        Self::Integer(value)
    }
}

impl From<property::double_property::DoubleProperty> for Property {
    fn from(value: property::double_property::DoubleProperty) -> Self {
        Self::Double(value)
    }
}

impl From<property::datetime_property::DateTimeProperty> for Property {
    fn from(value: property::datetime_property::DateTimeProperty) -> Self {
        Self::DateTime(value)
    }
}

impl From<property::string_property::StringProperty> for Property {
    fn from(value: property::string_property::StringProperty) -> Self {
        Self::String(value)
    }
}

impl From<property::Property> for Property {
    fn from(value: property::Property) -> Self {
        Self::Imported(value)
    }
}

fn concept_property<'a>(input: &'a str) -> CResult<&'a str, Property> {
    context(
        "ConcaptProperty",
        alt((
            into(property::string_property::string_property),
            into(property::boolean_property::boolean_property),
            into(property::integer_property::integer_property),
            into(property::long_property::long_property),
            into(property::datetime_property::datetime_property),
            into(property::double_property::double_property),
            into(property::imported_property),
        )),
    )(input)
}

pub fn declaration<'a>(input: &'a str) -> CResult<&'a str, Declaration> {
    let properties = context(
        "Properties",
        fold_many0(
            delimited(space0, concept_property, tuple((space0, line_ending))),
            Vec::new,
            |mut acc: Vec<_>, item: Property| {
                acc.push(item);
                acc
            },
        ),
    );

    let no_props = context(
        "NoProperties",
        tuple((char('{'), multispace0, char('}'))).map(|_| Vec::new()),
    );
    let props = context(
        "Properties",
        tuple((
            char('{'),
            space0,
            line_ending,
            properties,
            multispace0,
            char('}'),
        ))
        .map(|(_, _, _, props, _, _)| props),
    );

    let concept = tuple((
        keywords::concept,
        space1,
        token,
        space0,
        alt((props, no_props)),
    ))
    .map(|(_, _, name, _, props)| (name, props));

    context(
        "Declaration",
        concept.map(|(declaration_name, properties)| Declaration {
            name: declaration_name.to_string(),
            properties,
        }),
    )(input)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_concept_with_no_props() {
        let input = "concept MyConcept {}";
        assert_eq!(
            super::declaration(input),
            Ok((
                "",
                super::Declaration {
                    name: String::from("MyConcept"),
                    properties: Vec::new(),
                }
            )),
            "Should parse a declaration with no proeprties"
        );
    }

    #[test]
    fn test_concept_with_one_prop() {
        let input = "concept MyConcept {
          o String name
        }";
        assert_eq!(
            super::declaration(input),
            Ok((
                "",
                super::Declaration {
                    name: String::from("MyConcept"),
                    properties: vec![super::Property::String(
                        crate::parser::property::string_property::StringProperty {
                            name: String::from("name"),
                            is_array: false,
                            is_optional: false,
                            default_value: None,
                            regex_validator: None,
                            length_validator: None,
                        }
                    )],
                }
            )),
            "Should parse a declaration with one property"
        );
    }

    #[test]
    fn test_concept_with_several_props() {
        let input = "concept MyConcept {
          o String name
          o Boolean applied
          o Address address
        }";
        assert_eq!(
            super::declaration(input),
            Ok((
                "",
                super::Declaration {
                    name: String::from("MyConcept"),
                    properties: vec![
                        super::Property::String(
                            crate::parser::property::string_property::StringProperty {
                                name: String::from("name"),
                                is_array: false,
                                is_optional: false,
                                default_value: None,
                                regex_validator: None,
                                length_validator: None,
                            }
                        ),
                        super::Property::Boolean(
                            crate::parser::property::boolean_property::BooleanProperty {
                                name: String::from("applied"),
                                is_array: false,
                                is_optional: false,
                                default_value: None,
                            }
                        ),
                        super::Property::Imported(crate::parser::property::Property {
                            name: String::from("address"),
                            is_array: false,
                            is_optional: false,
                            class: String::from("Address")
                        })
                    ],
                }
            )),
            "Should parse a declaration with one property"
        );
    }
}
