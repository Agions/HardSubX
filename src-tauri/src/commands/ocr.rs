use serde::{Deserialize, Serialize};
use super::video::BoundingBox;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRConfig {
    pub engine: String,
    pub language: Vec<String>,
    pub confidence_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRResult {
    pub text: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

impl Default for OCRConfig {
    fn default() -> Self {
        Self {
            engine: "tesseract".to_string(),
            language: vec!["eng".to_string()],
            confidence_threshold: 0.7,
        }
    }
}

#[tauri::command]
pub async fn process_frame(
    frame_data: Vec<u8>,
    width: u32,
    height: u32,
    config: OCRConfig,
) -> Result<OCRResult, String> {
    tracing::info!("Processing frame with OCR engine: {}", config.engine);
    
    if frame_data.is_empty() {
        return Err("Frame data is empty".to_string());
    }
    
    if width == 0 || height == 0 {
        return Err("Invalid frame dimensions".to_string());
    }
    
    // OCR processing requires native library integration
    // For Tesseract: use tesseract-rs or call tesseract CLI
    // For PaddleOCR: use paddleocr-rs
    // This is a placeholder - actual implementation depends on chosen OCR engine
    
    tracing::info!("OCR processing requires native {} integration", config.engine);
    
    Err(format!(
        "OCR engine '{}' not yet integrated. Use Tesseract.js on frontend or integrate native {} library",
        config.engine, config.engine
    ))
}

#[tauri::command]
pub async fn process_roi(
    frame_data: Vec<u8>,
    width: u32,
    height: u32,
    roi: super::video::ROI,
    config: OCRConfig,
) -> Result<OCRResult, String> {
    tracing::info!("Processing ROI: {:?} with OCR engine: {}", roi, config.engine);
    
    if frame_data.is_empty() {
        return Err("Frame data is empty".to_string());
    }
    
    // Validate ROI dimensions
    if roi.width == 0 || roi.height == 0 {
        return Err("ROI has invalid dimensions".to_string());
    }
    
    // ROI-based OCR processing
    tracing::info!("ROI-based OCR requires native {} integration", config.engine);
    
    Err(format!(
        "OCR engine '{}' not yet integrated for ROI processing",
        config.engine
    ))
}
