#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(improper_ctypes)]
pub mod bindings_leptonica;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[link(name = "libtesseract", kind = "cdylib")]
pub mod bindings_tess_capi;

#[allow(non_upper_case_globals)]
pub mod bindings_tess_types;
