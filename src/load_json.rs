use json::{parse, JsonValue};
#[cfg(debug_assertions)]
pub fn load_json_file(path: &str) -> JsonValue {
    // Use file from disk in debug mode
    let s: String = std::fs::read_to_string(path).unwrap();
    parse(&s).unwrap()
}

#[cfg(not(debug_assertions))]
pub fn load_json_file(path: String) -> json::JsonValue {
    // Use baked-in version in release
    json::parse(include_str!(path)).unwrap()
}
