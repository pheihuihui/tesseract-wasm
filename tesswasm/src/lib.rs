mod generated;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn libmain() {
    let aa = generated::bindings_leptonica::LIBLEPT_MAJOR_VERSION;
    println!("{}", aa);
}
