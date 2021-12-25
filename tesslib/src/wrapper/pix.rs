use crate::generated::bindings_leptonica;
use bindings_leptonica::{pixDestroy, pixRead, pixReadMem};
use std::convert::{AsRef, TryInto};
use std::{ffi::CStr, num::TryFromIntError};
use thiserror::Error;

#[derive(Debug)]
pub struct Pix(*mut bindings_leptonica::Pix);

#[derive(Debug, Error, PartialEq)]
pub enum PixReadMemError {
    #[error("Pix::read_mem returned null")]
    NullPtr,
    #[error("Failed to convert image size")]
    ImageSizeConversion(#[from] TryFromIntError),
}

#[derive(Debug, Error)]
#[error("Pix::read returned null")]
pub struct PixReadError();

impl Drop for Pix {
    fn drop(&mut self) {
        unsafe {
            pixDestroy(&mut self.0);
        }
    }
}

impl AsRef<*mut bindings_leptonica::Pix> for Pix {
    fn as_ref(&self) -> &*mut bindings_leptonica::Pix {
        &self.0
    }
}

impl AsRef<bindings_leptonica::Pix> for Pix {
    fn as_ref(&self) -> &bindings_leptonica::Pix {
        unsafe { &*self.0 }
    }
}

impl Pix {
    pub unsafe fn new_from_pointer(ptr: *mut bindings_leptonica::Pix) -> Self {
        Self(ptr)
    }

    pub fn read(filename: &CStr) -> Result<Self, PixReadError> {
        let ptr = unsafe { pixRead(filename.as_ptr()) };
        if ptr.is_null() {
            Err(PixReadError())
        } else {
            Ok(Self(ptr))
        }
    }

    pub fn read_mem(img: &[u8]) -> Result<Self, PixReadMemError> {
        let ptr = unsafe { pixReadMem(img.as_ptr(), img.len().try_into()?) };
        if ptr.is_null() {
            Err(PixReadMemError::NullPtr)
        } else {
            Ok(Self(ptr))
        }
    }
}

#[test]
fn read_error_test() {
    let path = std::ffi::CString::new("fail").unwrap();
    assert!(Pix::read(&path).is_err());
}

#[test]
fn read_mem_error_test() {
    assert_eq!(Pix::read_mem(&[]).err(), Some(PixReadMemError::NullPtr));
}

#[test]
fn read_test() {
    let path = std::ffi::CString::new("image.png").unwrap();
    let pix = Pix::read(&path).unwrap();
    let lpix: &bindings_leptonica::Pix = pix.as_ref();
    assert_eq!(lpix.w, 200);
}
