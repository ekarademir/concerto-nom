pub mod parser;

use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric0;
use nom::character::complete::one_of;
use nom::error::VerboseError;
use nom::sequence::delimited;

fn main() {
    // let cto = "namespace com.example.foo@1.3.5-pre";
    // let parsed = parser::namespace_definition_parser::<VerboseError<&str>>(cto);
    let parsed = delimited(
        one_of::<&str, &str, VerboseError<&str>>("\'\""),
        alphanumeric0,
        one_of("'\""),
    )("\"amcik\"");
    println!("{:?}", parsed);
}
