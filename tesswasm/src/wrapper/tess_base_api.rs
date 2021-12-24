use crate::generated::bindings_tess_capi;
use thiserror;

use crate::wrapper::boxa::Boxa;
use crate::wrapper::pix::Pix;
use crate::wrapper::text::Text;
use bindings_tess_capi::{
    TessBaseAPIAllWordConfidences, TessBaseAPICreate, TessBaseAPIDelete, TessBaseAPIGetAltoText,
    TessBaseAPIGetComponentImages, TessBaseAPIGetHOCRText, TessBaseAPIGetInputImage,
    TessBaseAPIGetLSTMBoxText, TessBaseAPIGetSourceYResolution, TessBaseAPIGetTsvText,
    TessBaseAPIGetUTF8Text, TessBaseAPIGetWordStrBoxText, TessBaseAPIInit2, TessBaseAPIInit3,
    TessBaseAPIMeanTextConf, TessBaseAPIRecognize, TessBaseAPISetImage, TessBaseAPISetImage2,
    TessBaseAPISetRectangle, TessBaseAPISetSourceResolution, TessBaseAPISetVariable,
    TessDeleteIntArray, TessOcrEngineMode, TessPageIteratorLevel,
};
use std::convert::TryInto;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_int;
use std::ptr;
use std::slice;
use thiserror::Error;

use crate::wrapper::borrowed_pix;

#[derive(Debug)]
pub struct TessBaseApi(*mut bindings_tess_capi::TessBaseAPI);

unsafe impl Send for TessBaseApi {}

impl Drop for TessBaseApi {
    fn drop(&mut self) {
        unsafe { TessBaseAPIDelete(self.0) }
    }
}

impl Default for TessBaseApi {
    fn default() -> Self {
        Self::create()
    }
}

#[derive(Debug, Error)]
#[error("TessBaseApi failed to initialize")]
pub struct TessBaseApiInitError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to set variable")]
pub struct TessBaseApiSetVariableError();

#[derive(Debug, Error)]
#[error("TessBaseApi failed to recognize")]
pub struct TessBaseApiRecogniseError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_hocr_text returned null")]
pub struct TessBaseApiGetHocrTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_utf8_text returned null")]
pub struct TessBaseApiGetUtf8TextError();

#[derive(Debug, Error, PartialEq)]
pub enum TessBaseApiSetImageSafetyError {
    #[error("Image dimensions exceed computer memory")]
    DimensionsExceedMemory(),
    #[error("Image dimensions exceed image size")]
    DimensionsExceedImageSize(),
    #[error("Image width exceeds bytes per line")]
    ImageWidthExceedsBytesPerLine(),
}

#[derive(Debug, Error)]
#[error("TessBaseApi get_alto_text returned null")]
pub struct TessBaseApiGetAltoTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_tsv_text returned null")]
pub struct TessBaseApiGetTsvTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_lstm_box_text returned null")]
pub struct TessBaseApiGetLstmBoxTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_word_str_text returned null")]
pub struct TessBaseApiGetWordStrBoxTextError();

#[derive(Debug, Error)]
#[error("TessBaseApi get_component_images returned null")]
pub struct TessBaseApiGetComponentImagesError();

#[derive(Debug, Error)]
#[error("TessBaseApi all_word_confidences returned null")]
pub struct TessBaseApiAllWordConfidencesError();

pub struct AllWordConfidences(*mut c_int, usize);

impl AllWordConfidences {
    pub fn as_slice(&self) -> &[c_int] {
        self
    }

    pub fn as_slice_mut(&mut self) -> &mut [c_int] {
        self
    }
}

impl Deref for AllWordConfidences {
    type Target = [c_int];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.0, self.1) }
    }
}

impl DerefMut for AllWordConfidences {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.0, self.1) }
    }
}

impl Drop for AllWordConfidences {
    fn drop(&mut self) {
        unsafe {
            TessDeleteIntArray(self.0);
        }
    }
}

impl TessBaseApi {
    pub fn create() -> Self {
        Self(unsafe { TessBaseAPICreate() })
    }

    pub fn init_2(
        &mut self,
        datapath: Option<&CStr>,
        language: Option<&CStr>,
    ) -> Result<(), TessBaseApiInitError> {
        let ret = unsafe {
            TessBaseAPIInit3(
                self.0,
                datapath.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                language.map(CStr::as_ptr).unwrap_or_else(ptr::null),
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(TessBaseApiInitError {})
        }
    }

    pub fn init_4(
        &mut self,
        datapath: Option<&CStr>,
        language: Option<&CStr>,
        oem: TessOcrEngineMode,
    ) -> Result<(), TessBaseApiInitError> {
        let ret = unsafe {
            TessBaseAPIInit2(
                self.0,
                datapath.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                language.map(CStr::as_ptr).unwrap_or_else(ptr::null),
                oem,
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(TessBaseApiInitError {})
        }
    }

    pub fn set_image_2(&mut self, pix: &Pix) {
        unsafe {
            TessBaseAPISetImage2(self.0, *pix.as_ref());
        }
    }

    pub fn set_image(
        &mut self,
        image_data: &[u8],
        width: c_int,
        height: c_int,
        bytes_per_pixel: c_int,
        bytes_per_line: c_int,
    ) -> Result<(), TessBaseApiSetImageSafetyError> {
        let claimed_image_size: usize = (height * bytes_per_line)
            .try_into()
            .map_err(|_| TessBaseApiSetImageSafetyError::DimensionsExceedMemory())?;
        if claimed_image_size > image_data.len() {
            return Err(TessBaseApiSetImageSafetyError::DimensionsExceedImageSize());
        }
        match bytes_per_pixel {
            0 => {
                if width > bytes_per_line * 8 {
                    return Err(TessBaseApiSetImageSafetyError::ImageWidthExceedsBytesPerLine());
                }
            }
            _ => {
                if width * bytes_per_pixel > bytes_per_line {
                    return Err(TessBaseApiSetImageSafetyError::ImageWidthExceedsBytesPerLine());
                }
            }
        }
        unsafe {
            TessBaseAPISetImage(
                self.0,
                image_data.as_ptr(),
                width,
                height,
                bytes_per_pixel,
                bytes_per_line,
            );
        };
        Ok(())
    }

    pub fn set_source_resolution(&mut self, ppi: c_int) {
        unsafe {
            TessBaseAPISetSourceResolution(self.0, ppi);
        }
    }

    pub fn set_variable(
        &mut self,
        name: &CStr,
        value: &CStr,
    ) -> Result<(), TessBaseApiSetVariableError> {
        let ret = unsafe { TessBaseAPISetVariable(self.0, name.as_ptr(), value.as_ptr()) };
        match ret {
            1 => Ok(()),
            _ => Err(TessBaseApiSetVariableError {}),
        }
    }

    pub fn recognize(&mut self) -> Result<(), TessBaseApiRecogniseError> {
        let ret = unsafe { TessBaseAPIRecognize(self.0, ptr::null_mut()) };
        match ret {
            0 => Ok(()),
            _ => Err(TessBaseApiRecogniseError {}),
        }
    }

    pub fn get_utf8_text(&mut self) -> Result<Text, TessBaseApiGetUtf8TextError> {
        let ptr = unsafe { TessBaseAPIGetUTF8Text(self.0) };
        if ptr.is_null() {
            Err(TessBaseApiGetUtf8TextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    pub fn get_hocr_text(&mut self, page: c_int) -> Result<Text, TessBaseApiGetHocrTextError> {
        let ptr = unsafe { TessBaseAPIGetHOCRText(self.0, page) };
        if ptr.is_null() {
            Err(TessBaseApiGetHocrTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    pub fn get_input_image(&self) -> Option<borrowed_pix::BorrowedPix> {
        let ptr = unsafe { TessBaseAPIGetInputImage(self.0) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { borrowed_pix::BorrowedPix::new(ptr) })
        }
    }

    pub fn get_source_y_resolution(&self) -> c_int {
        unsafe { TessBaseAPIGetSourceYResolution(self.0) }
    }

    pub fn set_rectangle(&mut self, left: c_int, top: c_int, width: c_int, height: c_int) {
        unsafe { TessBaseAPISetRectangle(self.0, left, top, width, height) }
    }

    pub fn get_alto_text(
        &mut self,
        page_number: c_int,
    ) -> Result<Text, TessBaseApiGetAltoTextError> {
        let ptr = unsafe { TessBaseAPIGetAltoText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetAltoTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    pub fn get_tsv_text(&mut self, page_number: c_int) -> Result<Text, TessBaseApiGetTsvTextError> {
        let ptr = unsafe { TessBaseAPIGetTsvText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetTsvTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    pub fn get_lstm_box_text(
        &mut self,
        page_number: c_int,
    ) -> Result<Text, TessBaseApiGetLstmBoxTextError> {
        let ptr = unsafe { TessBaseAPIGetLSTMBoxText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetLstmBoxTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    pub fn get_word_str_box_text(
        &mut self,
        page_number: c_int,
    ) -> Result<Text, TessBaseApiGetWordStrBoxTextError> {
        let ptr = unsafe { TessBaseAPIGetWordStrBoxText(self.0, page_number) };
        if ptr.is_null() {
            Err(TessBaseApiGetWordStrBoxTextError {})
        } else {
            Ok(unsafe { Text::new(ptr) })
        }
    }

    pub fn mean_text_conf(&self) -> c_int {
        unsafe { TessBaseAPIMeanTextConf(self.0) }
    }

    pub fn all_word_confidences(
        &self,
    ) -> Result<AllWordConfidences, TessBaseApiAllWordConfidencesError> {
        let ptr = unsafe { TessBaseAPIAllWordConfidences(self.0) };
        if ptr.is_null() {
            Err(TessBaseApiAllWordConfidencesError {})
        } else {
            let mut end = ptr;
            unsafe {
                while *end != -1 {
                    end = end.add(1);
                }
                let len = end.offset_from(ptr);
                Ok(AllWordConfidences(ptr, len as usize))
            }
        }
    }

    pub fn get_component_images_1(
        &self,
        level: TessPageIteratorLevel,
        text_only: c_int,
    ) -> Result<Boxa, TessBaseApiGetComponentImagesError> {
        let ptr = unsafe {
            TessBaseAPIGetComponentImages(
                self.0,
                level,
                text_only,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if ptr.is_null() {
            Err(TessBaseApiGetComponentImagesError {})
        } else {
            Ok(unsafe { Boxa::new_from_pointer(ptr) })
        }
    }
}
