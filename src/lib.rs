pub mod exium;
pub mod utils;
use exium::Exium;
use serde_json;
extern crate wasm_bindgen;
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
#[wasm_bindgen(js_name = "readSync")]
pub fn read_sync(source: &str)-> String {
    utils::set_panic_hook();
    let exium: Exium = Exium::new();
    let contexts = exium.read(source,);
    let str = serde_json::to_string(&contexts);
    match str {
        Ok(value) => value,
        _ => panic!("error during parse")
    }
}
