use crate::parser::Model;

pub fn print(model: &Model) -> Result<String, Box<dyn std::error::Error>> {
    let s = serde_json::to_string_pretty(model)?;
    Ok(s)
}
