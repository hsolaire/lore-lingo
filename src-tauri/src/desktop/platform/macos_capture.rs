use crate::{
    desktop::{BackendError, BackendErrorCode},
    domain::captured_layout::MAX_CAPTURE_PIXELS,
};
use block2::RcBlock;
use core_graphics::access::ScreenCaptureAccess;
use objc2::{runtime::AnyClass, AnyThread};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::{CGBitmapContextCreate, CGColorSpace, CGContext, CGImage};
use objc2_foundation::{NSArray, NSError};
use objc2_screen_capture_kit::{
    SCContentFilter, SCScreenshotManager, SCShareableContent, SCStreamConfiguration,
};
use screenshots::image::RgbaImage;
use std::{sync::mpsc, time::Duration};

#[derive(Clone, Copy)]
pub enum CaptureTarget {
    Display(u32),
    Window(u32),
}

pub fn available() -> bool {
    AnyClass::get(c"SCScreenshotManager").is_some()
}

/// 仅传回有界像素缓冲区；框架对象在回调内使用，截图不写入文件。
pub fn capture(target: CaptureTarget) -> Result<RgbaImage, BackendError> {
    if !ScreenCaptureAccess.preflight() {
        return Err(BackendError::new(
            BackendErrorCode::PermissionDenied,
            "screen capture permission is required",
        ));
    }
    if !available() {
        return Err(BackendError::new(
            BackendErrorCode::UnsupportedCapability,
            "single-frame capture requires macOS 14",
        ));
    }
    // 步骤 1. 通过系统目录重新解析当前显示器或窗口，避免捕获已经关闭的目标。
    let (sender, receiver) = mpsc::channel();
    let callback = RcBlock::new(
        move |content: *mut SCShareableContent, error: *mut NSError| {
            if content.is_null() || !error.is_null() {
                let _ = sender.send(Err(BackendError::failure("capture catalog unavailable")));
                return;
            }
            // 系统只在回调期间保证这些对象有效；过滤器持有选中的目标对象。
            let filter = unsafe {
                let content = &*content;
                match target {
                    CaptureTarget::Window(id) => content
                        .windows()
                        .iter()
                        .find(|window| window.windowID() == id)
                        .map(|window| {
                            SCContentFilter::initWithDesktopIndependentWindow(
                                SCContentFilter::alloc(),
                                &window,
                            )
                        }),
                    CaptureTarget::Display(id) => content
                        .displays()
                        .iter()
                        .find(|display| display.displayID() == id)
                        .map(|display| {
                            // 实时区域预览排除本进程窗口，避免悬浮字幕被重复捕获。
                            let excluded: Vec<_> = content
                                .windows()
                                .iter()
                                .filter(|window| {
                                    window.owningApplication().is_some_and(|application| {
                                        application.processID() as u32 == std::process::id()
                                    })
                                })
                                .collect();
                            SCContentFilter::initWithDisplay_excludingWindows(
                                SCContentFilter::alloc(),
                                &display,
                                &NSArray::from_retained_slice(&excluded),
                            )
                        }),
                }
            };
            let Some(filter) = filter else {
                let _ = sender.send(Err(BackendError::new(
                    BackendErrorCode::SourceUnavailable,
                    "capture target no longer exists",
                )));
                return;
            };
            // 步骤 2. 按系统逻辑尺寸和缩放计算物理尺寸，限制像素总量。
            let (rect, scale) =
                unsafe { (filter.contentRect(), f64::from(filter.pointPixelScale())) };
            let width = (rect.size.width * scale).ceil();
            let height = (rect.size.height * scale).ceil();
            if !width.is_finite()
                || !height.is_finite()
                || width < 1.0
                || height < 1.0
                || width * height > MAX_CAPTURE_PIXELS as f64
            {
                let _ = sender.send(Err(BackendError::failure(
                    "capture dimensions outside limits",
                )));
                return;
            }
            let config = unsafe { SCStreamConfiguration::new() };
            unsafe {
                config.setWidth(width as usize);
                config.setHeight(height as usize);
                config.setShowsCursor(false);
            }
            let image_sender = sender.clone();
            let filter_lifetime = filter.clone();
            let config_lifetime = config.clone();
            let image_callback = RcBlock::new(move |image: *mut CGImage, error: *mut NSError| {
                let _keep_alive = (&filter_lifetime, &config_lifetime);
                let result = if image.is_null() || !error.is_null() {
                    Err(BackendError::failure("single-frame capture failed"))
                }
                // CGImage 属于系统回调，复制为自有缓冲区后才通过通道返回。
                else {
                    unsafe { copy_pixels(&*image) }
                };
                let _ = image_sender.send(result);
            });
            unsafe {
                SCScreenshotManager::captureImageWithFilter_configuration_completionHandler(
                    &filter,
                    &config,
                    Some(&image_callback),
                );
            }
        },
    );
    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(true, true, &callback);
    }
    // 步骤 3. 只在后台工作线程等待有界时间，超时回调自行释放持有对象。
    receiver
        .recv_timeout(Duration::from_secs(12))
        .map_err(|_| BackendError::failure("capture callback timed out"))?
}

fn copy_pixels(image: &CGImage) -> Result<RgbaImage, BackendError> {
    let width = CGImage::width(Some(image));
    let height = CGImage::height(Some(image));
    if width == 0 || height == 0 || (width as u64) * (height as u64) > MAX_CAPTURE_PIXELS {
        return Err(BackendError::failure("capture pixel buffer outside limits"));
    }
    let mut pixels = vec![0u8; width * height * 4];
    let color = CGColorSpace::new_device_rgb()
        .ok_or_else(|| BackendError::failure("RGB color space unavailable"))?;
    // 像素缓冲区在 CGContext 销毁之前保持原地址和长度；RGBA 使用预乘 alpha。
    let context = unsafe {
        CGBitmapContextCreate(
            pixels.as_mut_ptr().cast(),
            width,
            height,
            8,
            width * 4,
            Some(&color),
            1,
        )
    }
    .ok_or_else(|| BackendError::failure("bitmap context unavailable"))?;
    CGContext::draw_image(
        Some(&context),
        CGRect {
            origin: CGPoint { x: 0.0, y: 0.0 },
            size: CGSize {
                width: width as f64,
                height: height as f64,
            },
        },
        Some(image),
    );
    drop(context);
    RgbaImage::from_raw(width as u32, height as u32, pixels)
        .ok_or_else(|| BackendError::failure("invalid pixel layout"))
}
