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

use crate::parser::{common::keywords, property::internal::generic_property, CResult};

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub class: String,
    pub name: String,
    pub is_optional: bool,
    pub is_array: bool,
}

enum MetaProperty {
    Optional,
}

pub fn imported_property<'a>(input: &'a str) -> CResult<&'a str, Property> {
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
            super::imported_property("o MyType foo"),
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
            super::imported_property("o MyType[] foo"),
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
            super::imported_property("o MyType baz optional"),
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
            super::imported_property("o MyType[] baz optional"),
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
}
