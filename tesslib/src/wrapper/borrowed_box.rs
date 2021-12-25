use crate::generated::bindings_leptonica;

#[derive(Debug, PartialEq)]
pub struct BorrowedBox<'a>(&'a *mut bindings_leptonica::Box);

impl<'a> AsRef<bindings_leptonica::Box> for BorrowedBox<'a> {
    fn as_ref(&self) -> &bindings_leptonica::Box {
        unsafe { &**self.0 }
    }
}

impl<'a> BorrowedBox<'a> {
    pub unsafe fn new(b: &'a *mut bindings_leptonica::Box) -> Self {
        Self(b)
    }
}
