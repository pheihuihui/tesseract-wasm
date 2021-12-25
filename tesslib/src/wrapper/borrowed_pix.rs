use crate::generated::bindings_leptonica;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct BorrowedPix<'a> {
    raw: *mut bindings_leptonica::Pix,
    phantom: PhantomData<&'a *mut bindings_leptonica::Pix>,
}

impl<'a> AsRef<bindings_leptonica::Pix> for BorrowedPix<'a> {
    fn as_ref(&self) -> &bindings_leptonica::Pix {
        unsafe { &*self.raw }
    }
}

impl<'a> BorrowedPix<'a> {
    pub unsafe fn new(p: *mut bindings_leptonica::Pix) -> Self {
        Self {
            raw: p,
            phantom: PhantomData,
        }
    }
}
