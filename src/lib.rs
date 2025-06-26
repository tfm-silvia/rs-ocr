use pdf_extract;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ocr(bytes: &[u8]) -> String {
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();

    String::from(out)
}
