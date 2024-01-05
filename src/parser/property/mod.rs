mod internal;

pub mod boolean_property;
pub mod datetime_property;
pub mod double_property;
pub mod integer_property;
pub mod long_property;
pub mod string_property;

use nom::{
    character::complete::space1, error::context, multi::fold_many_m_n, sequence::preceded, Parser,
};
use serde_derive::Serialize;

use crate::parser::{common::keywords, property::internal::generic_property, CResult};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Property {
    #[serde(rename = "$class")]
    pub class: String,
    pub name: String,
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
    #[serde(rename = "isArray")]
    pub is_array: bool,
}

enum MetaProperty {
    Optional,
}

pub fn concept_property<'a>(input: &'a str) -> CResult<&'a str, Property> {
    let optional = preceded(space1, keywords::optional).map(|_| MetaProperty::Optional);

    context(
        "Property",
        generic_property
            .and(fold_many_m_n(
                0,
                1,
                optional,
                Vec::new,
                |mut acc: Vec<_>, meta_prop| {
                    acc.push(meta_prop);
                    acc
                },
            ))
            .map(|((class, property_name, is_array), meta_props)| {
                let mut prop = Property {
                    class: class.to_string(),
                    name: property_name.to_string(),
                    is_optional: false,
                    is_array,
                };

                for meta_prop in meta_props {
                    use MetaProperty::*;
                    match meta_prop {
                        Optional => prop.is_optional = true,
                    }
                }

                prop
            }),
    )(input)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_imported_property() {
        assert_eq!(
            super::concept_property("o MyType foo"),
            Ok((
                "",
                super::Property {
                    class: String::from("MyType"),
                    name: String::from("foo"),
                    is_optional: false,
                    is_array: false,
                }
            )),
            "Should parse imported type with no meta properties"
        );

        assert_eq!(
            super::concept_property("o MyType[] foo"),
            Ok((
                "",
                super::Property {
                    class: String::from("MyType"),
                    name: String::from("foo"),
                    is_optional: false,
                    is_array: true,
                }
            )),
            "Should parse imported type with array flag"
        );

        assert_eq!(
            super::concept_property("o MyType baz optional"),
            Ok((
                "",
                super::Property {
                    class: String::from("MyType"),
                    name: String::from("baz"),
                    is_optional: true,
                    is_array: false,
                }
            )),
            "Should parse imported type with optional flag"
        );

        assert_eq!(
            super::concept_property("o MyType[] baz optional"),
            Ok((
                "",
                super::Property {
                    class: String::from("MyType"),
                    name: String::from("baz"),
                    is_optional: true,
                    is_array: true,
                }
            )),
            "Should parse imported type with optional and array flag"
        );
    }

    #[test]
    fn test_serialize() {
        let a = super::Property {
            class: String::from("MyProperty"),
            name: String::from("aProperty"),
            is_array: false,
            is_optional: true,
        };

        assert_eq!(
            serde_json::json!({
              "$class": "MyProperty",
              "name": "aProperty",
              "isArray": false,
              "isOptional": true,
            }),
            serde_json::to_value(a).unwrap(),
        )
    }
}
