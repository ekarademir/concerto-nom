pub mod parser;

fn main() {
    let cto = "namespace com.example.foo@1.3.5-pre";
    let parsed = parser::namespace(cto);
    println!("{:?}", parsed);
}
