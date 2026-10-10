use objc2::{
    class, msg_send,
    rc::{Allocated, Retained},
    runtime::AnyObject,
};
use objc2_foundation::{NSArray, NSData, NSDictionary, NSError, NSRect, NSString};

use crate::{
    domain::captured_layout::{NormalizedRect, OcrObservation},
    services::capture_translation::CaptureTranslationError,
};

#[link(name = "Vision", kind = "framework")]
unsafe extern "C" {}

pub fn recognize_text(png: &[u8]) -> Result<Vec<OcrObservation>, CaptureTranslationError> {
    if png.is_empty() {
        return Err(CaptureTranslationError::OcrFailure);
    }

    // 此处的原生对象只在同步 Vision 请求期间存活，不会跨越适配器或 IPC 边界。
    unsafe {
        let data = NSData::with_bytes(png);
        let request: Retained<AnyObject> = msg_send![class!(VNRecognizeTextRequest), new];
        let _: () = msg_send![&request, setRecognitionLevel: 1isize];
        let _: () = msg_send![&request, setUsesLanguageCorrection: true];

        let options: Retained<NSDictionary<NSString, AnyObject>> = NSDictionary::new();
        let handler: Allocated<AnyObject> = msg_send![class!(VNImageRequestHandler), alloc];
        let handler: Retained<AnyObject> =
            msg_send![handler, initWithData: &*data, options: &*options];
        let requests: Retained<NSArray<AnyObject>> = NSArray::from_slice(&[&*request]);
        let mut error: Option<Retained<NSError>> = None;
        let succeeded: bool = msg_send![&handler, performRequests: &*requests, error: &mut error];
        if !succeeded {
            return Err(CaptureTranslationError::OcrFailure);
        }

        let results: *mut NSArray<AnyObject> = msg_send![&request, results];
        let Some(results) = results.as_ref() else {
            return Ok(Vec::new());
        };

        let mut observations = Vec::with_capacity(results.len());
        for observation in results {
            let candidates: *mut NSArray<AnyObject> =
                msg_send![&*observation, topCandidates: 1usize];
            let Some(candidates) = candidates.as_ref() else {
                continue;
            };
            let candidate: *mut AnyObject = msg_send![candidates, firstObject];
            let Some(candidate) = candidate.as_ref() else {
                continue;
            };
            let string: *mut NSString = msg_send![candidate, string];
            let Some(string) = string.as_ref() else {
                continue;
            };
            let confidence: f32 = msg_send![candidate, confidence];
            let rect: NSRect = msg_send![&*observation, boundingBox];
            let bounds = NormalizedRect {
                x: rect.origin.x,
                y: 1.0 - rect.origin.y - rect.size.height,
                width: rect.size.width,
                height: rect.size.height,
            };
            observations.push(OcrObservation {
                text: string.to_string(),
                bounds,
                confidence: confidence as f64,
            });
        }
        Ok(observations)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use screenshots::image::{DynamicImage, ImageFormat, Rgba, RgbaImage};

    use super::*;

    #[test]
    #[ignore = "需要在非沙箱 macOS 环境调用 Vision 框架"]
    fn vision_accepts_a_valid_blank_png_and_returns_an_empty_document() {
        let image = RgbaImage::from_pixel(96, 48, Rgba([255, 255, 255, 255]));
        let mut png = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(image)
            .write_to(&mut png, ImageFormat::Png)
            .expect("编码测试图片");

        let observations = recognize_text(&png.into_inner()).expect("执行 macOS Vision 烟雾测试");
        assert!(observations.is_empty());
    }
}
