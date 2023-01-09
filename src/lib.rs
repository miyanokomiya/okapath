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

#[wasm_bindgen(js_name = getTotalLeng)]
pub fn get_total_leng(d: &str) -> f64 {
    path::get_path_length(&parser::parse(d))
}

#[wasm_bindgen(js_name = getPointAtLength)]
pub fn get_point_at_length(d: &str, distance: f64) -> js_sys::Float64Array {
    js_sys::Float64Array::from(&vec![0.0, 0.0][..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path_length_cases() {
        assert_eq!(get_total_leng("M10 10 L 40 10 L40 50"), 70.0);
        assert_eq!(get_total_leng("M10 10 L 40 10 L40 50z"), 120.0);
    }
}
