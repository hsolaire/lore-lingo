use serde::{Deserialize, Serialize};

pub use super::translation::PublicError;

pub const MAX_CAPTURE_PIXELS: u64 = 32_000_000;
pub const MAX_CAPTURE_PNG_BYTES: usize = 24 * 1024 * 1024;
pub const MAX_TRANSLATION_BLOCKS: usize = 200;
pub const MAX_BLOCK_CHARS: usize = 4_000;
pub const MAX_TRANSLATION_INPUT_CHARS: usize = 100_000;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LogicalRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RegionBounds {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
}

impl RegionBounds {
    pub fn rect(&self, width: f64, height: f64) -> Option<LogicalRect> {
        let values = [self.x1, self.x2, self.y1, self.y2, width, height];
        if values.iter().any(|value| !value.is_finite())
            || self.x1 < 0.0
            || self.y1 < 0.0
            || self.x2 > width
            || self.y2 > height
            || self.x2 - self.x1 < 4.0
            || self.y2 - self.y1 < 4.0
        {
            return None;
        }
        Some(LogicalRect {
            x: self.x1,
            y: self.y1,
            width: self.x2 - self.x1,
            height: self.y2 - self.y1,
        })
    }
}

#[derive(Clone, Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CapturedRegion {
    pub revision: u64,
    pub monitor_id: String,
    pub logical_width: f64,
    pub logical_height: f64,
    pub bounds: RegionBounds,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCapturedRegionRequest {
    pub revision: u64,
    pub bounds: RegionBounds,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionPreview {
    pub revision: u64,
    pub image_png_b64: String,
    pub image_width: u32,
    pub image_height: u32,
    pub captured_at_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl NormalizedRect {
    pub fn validated(self) -> Option<Self> {
        let values = [self.x, self.y, self.width, self.height];
        if values.iter().any(|value| !value.is_finite()) || self.width <= 0.0 || self.height <= 0.0
        {
            return None;
        }

        let x = self.x.clamp(0.0, 1.0);
        let y = self.y.clamp(0.0, 1.0);
        let end_x = (self.x + self.width).clamp(0.0, 1.0);
        let end_y = (self.y + self.height).clamp(0.0, 1.0);
        let width = end_x - x;
        let height = end_y - y;
        (width > 0.0 && height > 0.0).then_some(Self {
            x,
            y,
            width,
            height,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutBlockKind {
    Title,
    Paragraph,
    ListItem,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayoutBlock {
    pub id: String,
    pub kind: LayoutBlockKind,
    pub bounds: NormalizedRect,
    pub source_text: String,
    pub translated_text: Option<String>,
    pub reading_order: u32,
    pub confidence: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDocument {
    pub image_width: u32,
    pub image_height: u32,
    pub image_png_b64: String,
    pub source_language: String,
    pub target_language: String,
    pub blocks: Vec<LayoutBlock>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TranslationConfig {
    pub endpoint: String,
    pub model: String,
    pub api_key: String,
    pub source_language: String,
    pub target_language: String,
}

impl std::fmt::Debug for TranslationConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TranslationConfig")
            .field("endpoint", &self.endpoint)
            .field("model", &self.model)
            .field("api_key", &"[已隐藏]")
            .field("source_language", &self.source_language)
            .field("target_language", &self.target_language)
            .finish()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartCaptureTranslationRequest {
    pub translation: TranslationConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StartCaptureTranslationResponse {
    pub session_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompleteCaptureTranslationRequest {
    pub session_id: String,
    pub monitor_id: String,
    pub rect: LogicalRect,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CancelCaptureTranslationRequest {
    pub session_id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureWindowTranslationRequest {
    pub source_id: String,
    pub catalog_revision: u64,
    pub translation: TranslationConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelectorPayload {
    pub session_id: String,
    pub monitor_id: String,
    pub image_png_b64: String,
    pub logical_width: f64,
    pub logical_height: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureTranslationState {
    Selecting,
    Processing,
    Empty,
    Success,
    Error,
    Cancelled,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CaptureTranslationEvent {
    pub session_id: String,
    pub state: CaptureTranslationState,
    pub document: Option<LayoutDocument>,
    pub error: Option<PublicError>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OcrObservation {
    pub text: String,
    pub bounds: NormalizedRect,
    pub confidence: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicalCrop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn logical_to_physical_crop(
    rect: &LogicalRect,
    scale_factor: f64,
    physical_width: u32,
    physical_height: u32,
) -> Option<PhysicalCrop> {
    if !scale_factor.is_finite() || scale_factor <= 0.0 {
        return None;
    }
    let values = [rect.x, rect.y, rect.width, rect.height];
    if values.iter().any(|value| !value.is_finite())
        || rect.x < 0.0
        || rect.y < 0.0
        || rect.width <= 0.0
        || rect.height <= 0.0
    {
        return None;
    }

    let start_x = (rect.x * scale_factor).floor().max(0.0);
    let start_y = (rect.y * scale_factor).floor().max(0.0);
    let end_x = ((rect.x + rect.width) * scale_factor)
        .ceil()
        .min(physical_width as f64);
    let end_y = ((rect.y + rect.height) * scale_factor)
        .ceil()
        .min(physical_height as f64);

    if start_x >= physical_width as f64
        || start_y >= physical_height as f64
        || end_x <= start_x
        || end_y <= start_y
    {
        return None;
    }

    Some(PhysicalCrop {
        x: start_x as u32,
        y: start_y as u32,
        width: (end_x - start_x) as u32,
        height: (end_y - start_y) as u32,
    })
}

pub fn build_layout_blocks(observations: Vec<OcrObservation>) -> Vec<LayoutBlock> {
    let mut observations: Vec<_> = observations
        .into_iter()
        .filter_map(|observation| {
            let text = observation.text.trim().to_string();
            let bounds = observation.bounds.validated()?;
            (!text.is_empty()).then_some(OcrObservation {
                text,
                bounds,
                confidence: observation.confidence.clamp(0.0, 1.0),
            })
        })
        .collect();

    observations.sort_by(|left, right| {
        left.bounds
            .y
            .total_cmp(&right.bounds.y)
            .then_with(|| left.bounds.x.total_cmp(&right.bounds.x))
    });

    let mut heights: Vec<f64> = observations
        .iter()
        .map(|observation| observation.bounds.height)
        .collect();
    heights.sort_by(f64::total_cmp);
    let median_height = heights
        .get(heights.len().saturating_sub(1) / 2)
        .copied()
        .unwrap_or(0.0);

    observations
        .into_iter()
        .enumerate()
        .map(|(index, observation)| LayoutBlock {
            id: format!("block-{index:04}"),
            kind: classify_block(&observation.text, observation.bounds.height, median_height),
            bounds: observation.bounds,
            source_text: observation.text,
            translated_text: None,
            reading_order: index as u32,
            confidence: observation.confidence,
        })
        .collect()
}

fn classify_block(text: &str, height: f64, median_height: f64) -> LayoutBlockKind {
    let trimmed = text.trim_start();
    let is_list_item = ["- ", "* ", "• ", "· "]
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
        || trimmed.split_once('.').is_some_and(|(prefix, _)| {
            !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit())
        });

    if is_list_item {
        LayoutBlockKind::ListItem
    } else if median_height > 0.0 && height >= median_height * 1.35 && text.chars().count() <= 120 {
        LayoutBlockKind::Title
    } else {
        LayoutBlockKind::Paragraph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crop_uses_floor_start_and_ceil_end() {
        let crop = logical_to_physical_crop(
            &LogicalRect {
                x: 10.2,
                y: 5.1,
                width: 20.2,
                height: 10.2,
            },
            2.0,
            200,
            100,
        )
        .expect("valid crop");

        assert_eq!(
            crop,
            PhysicalCrop {
                x: 20,
                y: 10,
                width: 41,
                height: 21
            }
        );
    }

    #[test]
    fn crop_rejects_empty_or_outside_regions() {
        assert!(logical_to_physical_crop(
            &LogicalRect {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 10.0
            },
            2.0,
            100,
            100,
        )
        .is_none());
        assert!(logical_to_physical_crop(
            &LogicalRect {
                x: 100.0,
                y: 0.0,
                width: 10.0,
                height: 10.0
            },
            1.0,
            100,
            100,
        )
        .is_none());
    }

    #[test]
    fn blocks_are_sorted_and_classified_with_stable_ids() {
        let blocks = build_layout_blocks(vec![
            OcrObservation {
                text: "- second".into(),
                bounds: NormalizedRect {
                    x: 0.1,
                    y: 0.5,
                    width: 0.3,
                    height: 0.08,
                },
                confidence: 0.8,
            },
            OcrObservation {
                text: "Heading".into(),
                bounds: NormalizedRect {
                    x: 0.1,
                    y: 0.1,
                    width: 0.5,
                    height: 0.16,
                },
                confidence: 1.2,
            },
        ]);

        assert_eq!(blocks[0].id, "block-0000");
        assert_eq!(blocks[0].kind, LayoutBlockKind::Title);
        assert_eq!(blocks[0].confidence, 1.0);
        assert_eq!(blocks[1].kind, LayoutBlockKind::ListItem);
    }
}
