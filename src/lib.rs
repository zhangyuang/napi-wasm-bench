#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "napi")]
use napi_derive::napi;

// Shared core function implementation
fn process_string_internal(input: &str) {
    // Here, a simple string processing function is implemented
    // For example: reverse the string and convert it to uppercase
    input.chars().rev().collect::<String>().to_uppercase();
}

// WASM export
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn process_string_wasm(input: &str) {
    process_string_internal(input);
}

// N-API export
#[cfg(feature = "napi")]
#[napi]
pub fn process_string_napi(input: String) {
    process_string_internal(&input);
}
