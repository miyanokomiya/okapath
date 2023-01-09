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
pub fn getPathLength(d: &str) -> f64 {
    path::get_path_length(&parser::parse(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path_length_cases() {
        assert_eq!(getPathLength("M10 10 L 40 10 L40 50"), 70.0);
        assert_eq!(getPathLength("M10 10 L 40 10 L40 50z"), 120.0);
    }
}
