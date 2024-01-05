pub mod parser;
pub mod serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cto = "
    namespace com.example.foo@1.3.5-pre

    concept Person {
      o String name
      o Integer age optional
      o Address mainAddress
    }

    concept Address {
      o String street
      o Integer number optional
      o String city default=\"Dublin\"
    }

    ";
    let (_, parsed) = parser::model(cto)?;

    println!("{}", serialize::print(&parsed)?);

    Ok(())
}
