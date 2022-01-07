use tesslib::ocr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ocr2() -> String {
    let res = ocr("", "eng");
    if let Ok(txt) = res {
        txt
    } else {
        "none".to_string()
    }
}
