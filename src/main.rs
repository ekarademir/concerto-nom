pub mod parser;

use nom::error::VerboseError;

fn main() {
    let cto = "namespace com.example.foo@1.3.5-pre";
    let parsed = parser::namespace_parser::<VerboseError<&str>>(cto);
    println!("{:?}", parsed);
}
