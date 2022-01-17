pub mod generated;
pub mod wrapper;

use thiserror::Error;

use std::ffi::CString;
use std::ffi::NulError;
use std::os::raw::c_int;
use std::str;

use wrapper::tess_base_api::TessBaseApi;

use generated::bindings_tess_capi::{
    TessOcrEngineMode, TessOcrEngineMode_OEM_DEFAULT, TessOcrEngineMode_OEM_LSTM_ONLY,
    TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED, TessOcrEngineMode_OEM_TESSERACT_ONLY,
};

#[derive(Debug, Error)]
pub enum InitializeError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("TessBaseApi failed to initialize")]
    TessBaseAPIInitError(#[from] wrapper::tess_base_api::TessBaseApiInitError),
}

#[derive(Debug, Error)]
pub enum SetImageError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("Failed to read image")]
    PixReadError(#[from] wrapper::pix::PixReadError),
}

#[derive(Debug, Error)]
pub enum SetVariableError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("TessBaseApi failed to set variable")]
    TessBaseAPISetVariableError(#[from] wrapper::tess_base_api::TessBaseApiSetVariableError),
}

#[derive(Debug, Error)]
pub enum TesseractError {
    #[error("Failed to set language")]
    InitializeError(#[from] InitializeError),
    #[error("Failed to set image")]
    SetImageError(#[from] SetImageError),
    #[error("Errored whilst recognizing")]
    RecognizeError(#[from] wrapper::tess_base_api::TessBaseApiRecogniseError),
    #[error("Errored whilst getting text")]
    GetTextError(#[from] wrapper::tess_base_api::TessBaseApiGetUtf8TextError),
    #[error("Errored whilst getting HOCR text")]
    GetHOCRTextError(#[from] wrapper::tess_base_api::TessBaseApiGetHocrTextError),
    #[error("Errored whilst setting frame")]
    SetFrameError(#[from] wrapper::tess_base_api::TessBaseApiSetImageSafetyError),
    #[error("Errored whilst setting image from mem")]
    SetImgFromMemError(#[from] wrapper::pix::PixReadMemError),
    #[error("Errored whilst setting variable")]
    SetVariableError(#[from] SetVariableError),
}

pub enum OcrEngineMode {
    /// Run Tesseract only - fastest; deprecated
    Default,
    /// Run just the LSTM line recognizer.
    LstmOnly,
    /// Run the LSTM recognizer, but allow fallback
    /// to Tesseract when things get difficult.
    /// deprecated
    TesseractLstmCombined,
    /// Specify this mode,
    /// to indicate that any of the above modes
    /// should be automatically inferred from the
    /// variables in the language-specific config,
    /// command-line configs, or if not specified
    /// in any of the above should be set to the
    /// default OEM_TESSERACT_ONLY.
    TesseractOnly,
}

impl OcrEngineMode {
    fn to_value(&self) -> TessOcrEngineMode {
        match *self {
            OcrEngineMode::Default => TessOcrEngineMode_OEM_DEFAULT,
            OcrEngineMode::LstmOnly => TessOcrEngineMode_OEM_LSTM_ONLY,
            OcrEngineMode::TesseractLstmCombined => TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED,
            OcrEngineMode::TesseractOnly => TessOcrEngineMode_OEM_TESSERACT_ONLY,
        }
    }
}

pub struct Tesseract(TessBaseApi);

impl Tesseract {
    pub fn new(datapath: Option<&str>, language: Option<&str>) -> Result<Self, InitializeError> {
        let mut tess = Tesseract(TessBaseApi::create());
        let datapath = match datapath {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };
        let language = match language {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };

        tess.0.init_2(datapath.as_deref(), language.as_deref())?;
        Ok(tess)
    }

    pub fn new_with_oem(
        datapath: Option<&str>,
        language: Option<&str>,
        oem: OcrEngineMode,
    ) -> Result<Self, InitializeError> {
        let mut tess = Tesseract(TessBaseApi::create());
        let datapath = match datapath {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };
        let language = match language {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };

        tess.0
            .init_4(datapath.as_deref(), language.as_deref(), oem.to_value())?;
        Ok(tess)
    }

    pub fn set_image(mut self, filename: &str) -> Result<Self, SetImageError> {
        let pix = wrapper::pix::Pix::read(&CString::new(filename)?)?;
        self.0.set_image_2(&pix);
        Ok(self)
    }
    pub fn set_frame(
        mut self,
        frame_data: &[u8],
        width: i32,
        height: i32,
        bytes_per_pixel: i32,
        bytes_per_line: i32,
    ) -> Result<Self, wrapper::tess_base_api::TessBaseApiSetImageSafetyError> {
        self.0
            .set_image(frame_data, width, height, bytes_per_pixel, bytes_per_line)?;
        Ok(self)
    }
    pub fn set_image_from_mem(mut self, img: &[u8]) -> Result<Self, wrapper::pix::PixReadMemError> {
        let pix = wrapper::pix::Pix::read_mem(img)?;
        self.0.set_image_2(&pix);
        Ok(self)
    }

    pub fn set_source_resolution(mut self, ppi: i32) -> Self {
        self.0.set_source_resolution(ppi);
        self
    }

    pub fn set_variable(mut self, name: &str, value: &str) -> Result<Self, SetVariableError> {
        self.0
            .set_variable(&CString::new(name)?, &CString::new(value)?)?;
        Ok(self)
    }
    pub fn recognize(mut self) -> Result<Self, wrapper::tess_base_api::TessBaseApiRecogniseError> {
        self.0.recognize()?;
        Ok(self)
    }
    pub fn get_text(
        &mut self,
    ) -> Result<String, wrapper::tess_base_api::TessBaseApiGetUtf8TextError> {
        Ok(self
            .0
            .get_utf8_text()?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }
    pub fn mean_text_conf(&mut self) -> i32 {
        self.0.mean_text_conf()
    }

    pub fn get_hocr_text(
        &mut self,
        page: c_int,
    ) -> Result<String, wrapper::tess_base_api::TessBaseApiGetHocrTextError> {
        Ok(self
            .0
            .get_hocr_text(page)?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }
}

pub fn ocr(filename: &str, language: &str) -> Result<String, TesseractError> {
    Ok(Tesseract::new(None, Some(language))?
        .set_image(filename)?
        .recognize()?
        .get_text()?)
}

pub fn ocr_from_frame(
    frame_data: &[u8],
    width: i32,
    height: i32,
    bytes_per_pixel: i32,
    bytes_per_line: i32,
    language: &str,
) -> Result<String, TesseractError> {
    Ok(Tesseract::new(None, Some(language))?
        .set_frame(frame_data, width, height, bytes_per_pixel, bytes_per_line)?
        .recognize()?
        .get_text()?)
}
