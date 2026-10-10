use super::capture_translation::{encode_png, CaptureTranslationError};
#[cfg(target_os = "macos")]
use crate::domain::captured_layout::logical_to_physical_crop;
use crate::domain::captured_layout::{
    CapturedRegion, RegionBounds, RegionPreview, UpdateCapturedRegionRequest,
};
use base64::{engine::general_purpose, Engine as _};
#[cfg(target_os = "macos")]
use screenshots::image::imageops;
use screenshots::image::RgbaImage;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default)]
pub struct RegionCaptureService {
    current: Mutex<Option<CapturedRegion>>,
    next_revision: AtomicU64,
    #[cfg(target_os = "macos")]
    frame_gate: Mutex<()>,
}

impl RegionCaptureService {
    pub fn current(&self) -> Result<Option<CapturedRegion>, CaptureTranslationError> {
        Ok(self
            .current
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?
            .clone())
    }

    pub fn remember(
        &self,
        monitor_id: String,
        width: f64,
        height: f64,
        bounds: RegionBounds,
    ) -> Result<CapturedRegion, CaptureTranslationError> {
        bounds
            .rect(width, height)
            .ok_or(CaptureTranslationError::InvalidRegion)?;
        let region = CapturedRegion {
            revision: self.next_revision.fetch_add(1, Ordering::Relaxed) + 1,
            monitor_id,
            logical_width: width,
            logical_height: height,
            bounds,
        };
        *self
            .current
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)? = Some(region.clone());
        Ok(region)
    }

    pub fn update(
        &self,
        request: UpdateCapturedRegionRequest,
    ) -> Result<CapturedRegion, CaptureTranslationError> {
        let mut current = self
            .current
            .lock()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        let region = current
            .as_mut()
            .ok_or(CaptureTranslationError::InvalidRegion)?;
        if region.revision != request.revision {
            return Err(CaptureTranslationError::StaleTask);
        }
        request
            .bounds
            .rect(region.logical_width, region.logical_height)
            .ok_or(CaptureTranslationError::InvalidRegion)?;
        region.bounds = request.bounds;
        region.revision = self.next_revision.fetch_add(1, Ordering::Relaxed) + 1;
        Ok(region.clone())
    }

    #[cfg(target_os = "macos")]
    pub fn snapshot(&self, revision: u64) -> Result<RgbaImage, CaptureTranslationError> {
        // 步骤 1. 单帧请求使用独占门闩，重复请求立即失败，不排队也不持有区域状态锁。
        let _gate = self
            .frame_gate
            .try_lock()
            .map_err(|_| CaptureTranslationError::CaptureFailure)?;
        let region = self
            .current()?
            .ok_or(CaptureTranslationError::InvalidRegion)?;
        if region.revision != revision {
            return Err(CaptureTranslationError::StaleTask);
        }
        // 步骤 2. 重新查询显示器与缩放，捕获当前画面后裁剪；断开或越界必须明确失败。
        let image = {
            let screen = screenshots::Screen::all()
                .map_err(|_| CaptureTranslationError::CaptureFailure)?
                .into_iter()
                .find(|screen| screen.display_info.id.to_string() == region.monitor_id)
                .ok_or(CaptureTranslationError::InvalidRegion)?;
            let image =
                crate::desktop::capture_display_image(screen.display_info.id).map_err(|error| {
                    match error.to_public().code {
                        crate::desktop::DesktopErrorCode::PermissionDenied => {
                            CaptureTranslationError::PermissionDenied
                        }
                        crate::desktop::DesktopErrorCode::UnsupportedCapability => {
                            CaptureTranslationError::UnsupportedCapability
                        }
                        _ => CaptureTranslationError::CaptureFailure,
                    }
                })?;
            let scale = f64::from(screen.display_info.scale_factor);
            let rect = region
                .bounds
                .rect(
                    f64::from(image.width()) / scale,
                    f64::from(image.height()) / scale,
                )
                .ok_or(CaptureTranslationError::InvalidRegion)?;
            let crop = logical_to_physical_crop(&rect, scale, image.width(), image.height())
                .ok_or(CaptureTranslationError::InvalidRegion)?;
            imageops::crop_imm(&image, crop.x, crop.y, crop.width, crop.height).to_image()
        };
        // 步骤 3. 坐标改变后的旧帧不能成为有效结果。
        if self.current()?.as_ref().map(|value| value.revision) != Some(revision) {
            return Err(CaptureTranslationError::StaleTask);
        }
        Ok(image)
    }

    #[cfg(not(target_os = "macos"))]
    pub fn snapshot(&self, _revision: u64) -> Result<RgbaImage, CaptureTranslationError> {
        Err(CaptureTranslationError::UnsupportedCapability)
    }

    pub fn preview(&self, revision: u64) -> Result<RegionPreview, CaptureTranslationError> {
        let image = self.snapshot(revision)?;
        let (width, height) = (image.width(), image.height());
        let thumbnail = screenshots::image::DynamicImage::ImageRgba8(image)
            .thumbnail(640, 280)
            .to_rgba8();
        Ok(RegionPreview {
            revision,
            image_png_b64: general_purpose::STANDARD.encode(encode_png(&thumbnail)?),
            image_width: width,
            image_height: height,
            captured_at_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn bounds() -> RegionBounds {
        RegionBounds {
            x1: 10.0,
            x2: 120.0,
            y1: 20.0,
            y2: 80.0,
        }
    }
    #[test]
    fn coordinate_updates_are_atomic_and_reject_stale_or_outside_input() {
        let service = RegionCaptureService::default();
        let first = service
            .remember("opaque-monitor".into(), 300.0, 200.0, bounds())
            .expect("记住选区");
        let invalid = RegionBounds {
            x2: 400.0,
            ..bounds()
        };
        assert_eq!(
            service.update(UpdateCapturedRegionRequest {
                revision: first.revision,
                bounds: invalid
            }),
            Err(CaptureTranslationError::InvalidRegion)
        );
        assert_eq!(service.current().expect("读取").expect("存在选区"), first);
        let next = service
            .update(UpdateCapturedRegionRequest {
                revision: first.revision,
                bounds: RegionBounds {
                    x1: 30.0,
                    ..bounds()
                },
            })
            .expect("更新坐标");
        assert!(next.revision > first.revision);
        assert_eq!(
            service.update(UpdateCapturedRegionRequest {
                revision: first.revision,
                bounds: bounds()
            }),
            Err(CaptureTranslationError::StaleTask)
        );
        assert!(RegionBounds {
            x2: f64::NAN,
            ..bounds()
        }
        .rect(300.0, 200.0)
        .is_none());
        assert!(RegionBounds {
            y2: 19.0,
            ..bounds()
        }
        .rect(300.0, 200.0)
        .is_none());
    }
}
