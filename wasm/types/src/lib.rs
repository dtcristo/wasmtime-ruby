use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn void() {}

#[wasm_bindgen]
pub fn u32_u32(x: u32) -> u32 {
    x
}

#[wasm_bindgen]
pub fn i32_i32(x: i32) -> i32 {
    x
}

// #[wasm_bindgen]
// pub fn u64_u64(x: u64) -> u64 {
//     x
// }

// #[wasm_bindgen]
// pub fn i64_i64(x: i64) -> i64 {
//     x
// }

#[wasm_bindgen]
pub fn f32_f32(x: f32) -> f32 {
    x
}

#[wasm_bindgen]
pub fn f64_f64(x: f64) -> f64 {
    x
}

#[wasm_bindgen]
pub fn bool_bool(x: bool) -> bool {
    !x
}

#[wasm_bindgen]
pub fn str_string(x: &str) -> String {
    format!("Hello, {}!", x)
}
