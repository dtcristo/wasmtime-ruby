use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn void() {}

#[wasm_bindgen]
pub fn u8_u8(x: u8) -> u8 {
    x + 1
}

#[wasm_bindgen]
pub fn i8_i8(x: i8) -> i8 {
    x + 1
}

#[wasm_bindgen]
pub fn u16_u16(x: u16) -> u16 {
    x + 1
}

#[wasm_bindgen]
pub fn i16_i16(x: i16) -> i16 {
    x + 1
}

#[wasm_bindgen]
pub fn u32_u32(x: u32) -> u32 {
    x + 1
}

#[wasm_bindgen]
pub fn i32_i32(x: i32) -> i32 {
    x + 1
}

#[wasm_bindgen]
pub fn usize_usize(x: usize) -> usize {
    x + 1
}

#[wasm_bindgen]
pub fn isize_isize(x: isize) -> isize {
    x + 1
}

#[wasm_bindgen]
pub fn u64_u64(x: u64) -> u64 {
    x + 1
}

#[wasm_bindgen]
pub fn i64_i64(x: i64) -> i64 {
    x + 1
}

// #[wasm_bindgen]
// pub fn u128_u128(x: u128) -> u128 {
//     x + 1
// }

// #[wasm_bindgen]
// pub fn i128_i128(x: i128) -> i128 {
//     x + 1
// }

#[wasm_bindgen]
pub fn f32_f32(x: f32) -> f32 {
    x * 2.0
}

#[wasm_bindgen]
pub fn f64_f64(x: f64) -> f64 {
    x * 2.0
}

#[wasm_bindgen]
pub fn bool_bool(x: bool) -> bool {
    !x
}

#[wasm_bindgen]
pub fn str_string(x: &str) -> String {
    format!("Hello, {}!", x)
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
