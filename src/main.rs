pub mod parser;

fn main() {
    let cto = "namespaces com.example.foo@1.3.5-pre";
    let parsed = parser::namespace_definition_parser(cto);

    println!("{:?}", parsed);
}
