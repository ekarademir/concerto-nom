use nom::{
    combinator::{into, opt},
    error::context,
    Parser,
};

use crate::parser::{common::boolean_value, CResult};

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanProperty {
    pub default_value: Option<bool>,
}

pub fn boolean_property<'a>(input: &'a str) -> CResult<&'a str, BooleanProperty> {
    context(
        "BooleanProperty",
        opt(boolean_default_value).map(|default_value| BooleanProperty { default_value }),
    )(input)
}

pub fn boolean_default_value<'a>(input: &'a str) -> CResult<&'a str, bool> {
    into(context("BooleanDefaultValue", boolean_value))(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_boolean_property() {}
}
