#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "napi")]
use napi_derive::napi;

// 共享的核心函数实现
fn process_string_internal(input: &str) {
    // 这里实现一个简单的字符串处理函数
    // 例如：将字符串反转并转为大写
    input.chars().rev().collect::<String>().to_uppercase();
}

// WASM 导出
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn process_string_wasm(input: &str) {
    process_string_internal(input);
}

// N-API 导出
#[cfg(feature = "napi")]
#[napi]
pub fn process_string_napi(input: String) {
    process_string_internal(&input);
}
