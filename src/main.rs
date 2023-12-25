pub mod parser;

use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::Parser;

fn main() {
    let cto = "namespace com.example.foo@1.3.5-pre";
    let parsed = parser::namespace(cto);
    println!("{:?}", parsed);
}
