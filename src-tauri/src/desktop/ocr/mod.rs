#[cfg(target_os = "macos")]
mod macos;

use crate::{
    domain::captured_layout::OcrObservation, services::capture_translation::CaptureTranslationError,
};

pub fn recognize_text(png: &[u8]) -> Result<Vec<OcrObservation>, CaptureTranslationError> {
    #[cfg(target_os = "macos")]
    {
        macos::recognize_text(png)
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = png;
        Err(CaptureTranslationError::UnsupportedCapability)
    }
}
