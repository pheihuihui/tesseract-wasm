mod generated;

mod wrapper;

use crate::generated::bindings_leptonica::{pixFreeData, pixRead};
use std::ffi::CStr;
use wasm_bindgen::prelude::*;

#[test]
#[wasm_bindgen]
pub fn image_size() {
    unsafe {
        let image =
            pixRead(CStr::from_bytes_with_nul_unchecked(b"../threat_captcha.png\0").as_ptr());
        println!("{}", (*image).w);
        println!("{}", (*image).h);
        assert_eq!((*image).w, 160);
        assert_eq!((*image).h, 75);
        pixFreeData(image);
    }
}
