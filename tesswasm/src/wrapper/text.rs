use crate::generated::bindings_tess_capi::TessDeleteText;

use std::convert::AsRef;
use std::ffi::CStr;
use std::os::raw::c_char;

pub struct Text(*mut c_char);

unsafe impl Send for Text {}

impl Drop for Text {
    fn drop(&mut self) {
        unsafe { TessDeleteText(self.0) }
    }
}

impl Text {
    pub unsafe fn new(raw: *mut c_char) -> Self {
        Self(raw)
    }
}

impl AsRef<CStr> for Text {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }
}
