pub mod parser;
pub mod path;
pub mod utils;
pub mod vector;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, okapath!");
}

#[wasm_bindgen]
pub fn getPathLength(d: String) -> f64 {
    path::get_path_length(&parser::parse(d))
}
