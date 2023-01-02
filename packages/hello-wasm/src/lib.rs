use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(s: &str) -> String {
    format!("Hello, {s}!")
}
