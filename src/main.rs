use serde_json::{from_str, Value};

fn main() {
    let json = "123:u32";
    let v: Value = from_str(json).unwrap();
    assert_eq!(v, Value::String(json.to_string()));
}
