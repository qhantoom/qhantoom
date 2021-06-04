use wasm_bindgen::prelude::*;

#[inline]
#[wasm_bindgen]
pub fn abort(e: &str) {
  // error::throw("error: {}", e)
}
