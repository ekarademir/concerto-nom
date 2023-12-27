pub mod parser;

use nom::error::{context, VerboseError};
use nom::Parser;

fn main() {
    let cto = "namespace com.example.foo@1.3.5-pre";
    let parsed = parser::namespace_definition_parser::<VerboseError<&str>>(cto);

    println!("{:?}", parsed);
}
