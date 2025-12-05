//! WASM bindings for avila-arrow

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmArray {
    // Placeholder
}

#[wasm_bindgen]
impl WasmArray {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn len(&self) -> usize {
        0
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
