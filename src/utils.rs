#[cfg(cli)]
use serde_json::Value;

#[cfg(cli)]
pub fn read_json(path: &str) -> Value{
    let json_str = std::fs::read_to_string(path).expect(&format!("file: {} not found", path));
    let json: Value = serde_json::from_str(&json_str).unwrap();
    json
}