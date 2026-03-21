use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCREngineConfig {
    pub engine: String,
    pub language: Vec<String>,
    pub confidence_threshold: f32,
    pub use_gpu: bool,
}

impl Default for OCREngineConfig {
    fn default() -> Self {
        Self {
            engine: "tesseract".to_string(),
            language: vec!["eng".to_string()],
            confidence_threshold: 0.7,
            use_gpu: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRResultItem {
    pub text: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRProcessResult {
    pub items: Vec<OCRResultItem>,
    pub full_text: String,
    pub language_detected: String,
    pub processing_time_ms: u64,
}

#[tauri::command]
pub async fn init_ocr_engine(config: OCREngineConfig) -> Result<String, String> {
    tracing::info!("Initializing OCR engine: {} with {:?}", config.engine, config.language);
    
    match config.engine.as_str() {
        "paddle" => {
            Err("PaddleOCR native integration not yet implemented".to_string())
        }
        "tesseract" => {
            // Tesseract is used via WASM on frontend
            Ok("tesseract-wasm".to_string())
        }
        "easyocr" => {
            Err("EasyOCR native integration not yet implemented".to_string())
        }
        _ => Err(format!("Unknown OCR engine: {}", config.engine))
    }
}

#[tauri::command]
pub async fn process_image_ocr(
    image_data: Vec<u8>,
    width: u32,
    height: u32,
    config: OCREngineConfig,
) -> Result<OCRProcessResult, String> {
    tracing::info!("Processing image with {} OCR engine", config.engine);
    
    if image_data.is_empty() {
        return Err("Image data is empty".to_string());
    }
    
    if width == 0 || height == 0 {
        return Err("Invalid image dimensions".to_string());
    }
    
    let start = std::time::Instant::now();
    
    // OCR processing requires native library integration
    // Native Rust OCR libraries are limited, so we recommend using Tesseract.js on frontend
    
    Err(format!(
        "Native OCR processing not yet implemented. Use Tesseract.js on frontend for {} engine",
        config.engine
    ))
}

#[tauri::command]
pub async fn process_roi_ocr(
    image_data: Vec<u8>,
    width: u32,
    height: u32,
    roi_x: u32,
    roi_y: u32,
    roi_width: u32,
    roi_height: u32,
    config: OCREngineConfig,
) -> Result<OCRProcessResult, String> {
    tracing::info!("Processing ROI ({}, {}) {}x{} with {} engine", 
        roi_x, roi_y, roi_width, roi_height, config.engine);
    
    if image_data.is_empty() {
        return Err("Image data is empty".to_string());
    }
    
    if roi_width == 0 || roi_height == 0 {
        return Err("ROI has invalid dimensions".to_string());
    }
    
    let start = std::time::Instant::now();
    
    Err(format!(
        "ROI-based OCR processing not yet implemented for {} engine",
        config.engine
    ))
}

#[tauri::command]
pub fn get_available_ocr_engines() -> HashMap<String, bool> {
    let mut engines = HashMap::new();
    // Tesseract.js is available via frontend WASM
    engines.insert("tesseract".to_string(), true);
    // Native engines require library integration
    engines.insert("paddle".to_string(), false);
    engines.insert("easyocr".to_string(), false);
    engines
}

#[tauri::command]
pub fn get_ocr_engine_info(engine: String) -> Result<serde_json::Value, String> {
    match engine.as_str() {
        "tesseract" => Ok(serde_json::json!({
            "name": "Tesseract.js",
            "type": "wasm",
            "languages": ["eng", "chi_sim", "chi_tra", "jpn", "kor", "fra", "deu", "spa", "por", "ita"],
            "gpu_support": false,
            "accuracy": "medium",
            "speed": "fast",
            "description": "Pure JavaScript OCR using WebAssembly. Works in browser without native installation."
        })),
        "paddle" => Ok(serde_json::json!({
            "name": "PaddleOCR",
            "type": "native",
            "languages": ["ch", "en", "ja", "ko", "fr", "de", "es", "ru", "ar"],
            "gpu_support": true,
            "accuracy": "high",
            "speed": "medium",
            "description": "BAIDU's OCR engine. High accuracy, especially for Chinese. Requires native Rust bindings."
        })),
        "easyocr" => Ok(serde_json::json!({
            "name": "EasyOCR",
            "type": "native",
            "languages": ["ch", "en", "ja", "ko", "fr", "de", "es", "it", "pt", "ru"],
            "gpu_support": true,
            "accuracy": "high",
            "speed": "slow",
            "description": "Python-based OCR with broad language support. Requires Python integration."
        })),
        _ => Err(format!("Unknown OCR engine: {}. Available: tesseract, paddle, easyocr", engine))
    }
}
