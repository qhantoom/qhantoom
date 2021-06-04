use wasm_bindgen::prelude::*;

use qhantoom::front;

#[inline]
#[wasm_bindgen]
pub fn tokenize(src: &str) -> String {
  match front::tokenizer::tokenize_capsule_from_source(src) {
    Ok(tokens) => format!("{:#?}", tokens),
    _ => format!("tokenize error."),
  }
}

#[inline]
#[wasm_bindgen]
pub fn parse(src: &str) -> String {
  match front::parser::parse_capsule_from_source(src) {
    Ok(_astree) => format!("parse function not implemented yet"),
    _ => format!("tokenize error."),
  }
}

#[inline]
#[wasm_bindgen]
pub fn analyze(src: &str) -> String {
  match front::analyzer::analyze_capsule_from_source(src) {
    Ok(_result) => format!("analyze function not implemented yet"),
    _ => format!("tokenize error."),
  }
}

#[inline]
#[wasm_bindgen]
pub fn interpret(src: &str) -> String {
  match front::interpreter::interpret_capsule_from_source(src) {
    Ok(_object) => format!("interpret function not implemented yet"),
    _ => format!("tokenize error."),
  }
}
