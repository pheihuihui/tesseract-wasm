use crate::generated::bindings_leptonica;
use bindings_leptonica::{boxCreateValid, boxDestroy, l_int32};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Box(*mut bindings_leptonica::Box);

#[derive(Debug, Error)]
#[error("Box::create_valid returned null")]
pub struct BoxCreateValidError();

impl Drop for Box {
    fn drop(&mut self) {
        unsafe {
            boxDestroy(&mut self.0);
        }
    }
}

impl AsRef<bindings_leptonica::Box> for Box {
    fn as_ref(&self) -> &bindings_leptonica::Box {
        unsafe { &*self.0 }
    }
}

impl Box {
    pub fn new(
        x: l_int32,
        y: l_int32,
        w: l_int32,
        h: l_int32,
    ) -> Result<Self, BoxCreateValidError> {
        Self::create_valid(x, y, w, h)
    }

    pub fn create_valid(
        x: l_int32,
        y: l_int32,
        w: l_int32,
        h: l_int32,
    ) -> Result<Self, BoxCreateValidError> {
        let ptr = unsafe { boxCreateValid(x, y, w, h) };
        if ptr.is_null() {
            Err(BoxCreateValidError())
        } else {
            Ok(Self(ptr))
        }
    }
}

#[test]
fn create_valid_test() {
    let r#box = Box::create_valid(1, 2, 3, 4).unwrap();
    let lbox: &bindings_leptonica::Box = r#box.as_ref();
    assert_eq!(lbox.w, 3);
}

#[test]
fn create_invalid_test() {
    assert!(Box::create_valid(1, 2, 3, -4).is_err())
}

#[test]
fn testtest() {
    assert_eq!(1, 1);
}
