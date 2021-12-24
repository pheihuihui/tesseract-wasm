use crate::generated::bindings_leptonica;
use crate::wrapper::borrowed_box;

use bindings_leptonica::{boxaCreate, boxaDestroy, l_int32};

#[derive(Debug, PartialEq)]
pub struct Boxa(*mut bindings_leptonica::Boxa);

impl Drop for Boxa {
    fn drop(&mut self) {
        unsafe {
            boxaDestroy(&mut self.0);
        }
    }
}

impl AsRef<bindings_leptonica::Boxa> for Boxa {
    fn as_ref(&self) -> &bindings_leptonica::Boxa {
        unsafe { &*self.0 }
    }
}

impl Boxa {
    pub unsafe fn new_from_pointer(p: *mut bindings_leptonica::Boxa) -> Self {
        Self(p)
    }

    pub fn create(n: l_int32) -> Option<Boxa> {
        let ptr = unsafe { boxaCreate(n) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    pub fn get(&self, i: isize) -> Option<borrowed_box::BorrowedBox> {
        let lboxa: &bindings_leptonica::Boxa = self.as_ref();
        if lboxa.n < std::convert::TryFrom::try_from(i).ok()? {
            None
        } else {
            unsafe { Some(borrowed_box::BorrowedBox::new(&*lboxa.box_.offset(i))) }
        }
    }
}

#[test]
fn create_valid_test() {
    let boxa = Boxa::create(4).unwrap();
    let lboxa: &bindings_leptonica::Boxa = boxa.as_ref();
    assert_eq!(lboxa.nalloc, 4);
    assert_eq!(lboxa.n, 0);
}
