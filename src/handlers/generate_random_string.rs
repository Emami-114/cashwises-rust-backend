use chrono::format::Item::Numeric;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_json::Value::Number;
use std::ffi::c_char;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = rng.gen_range(1111..9999).to_string();
    // .sample_iter(&Number)
    // .take(length)
    // // .map(char::from)
    // .collect();
    random_string
}
