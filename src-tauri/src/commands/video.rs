use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub duration: f64,
    pub fps: f64,
    pub total_frames: u64,
    pub codec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub index: u64,
    pub timestamp: f64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROI {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub roi_type: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractOptions {
    pub scene_threshold: f32,
    pub frame_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Default for ROI {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default".to_string(),
            roi_type: "bottom".to_string(),
            x: 0,
            y: 0,
            width: 1920,
            height: 100,
            enabled: true,
        }
    }
}

#[tauri::command]
pub async fn get_video_metadata(path: String) -> Result<VideoMetadata, String> {
    tracing::info!("Getting metadata for: {}", path);
    
    let path_obj = Path::new(&path);
    
    if !path_obj.exists() {
        return Err(format!("File not found: {}", path));
    }
    
    // Return metadata from file stats as placeholder
    // Real implementation would use FFmpeg or OpenCV
    let metadata = match std::fs::metadata(&path) {
        Ok(meta) => {
            let file_size = meta.len();
            // Estimate duration based on file size (rough estimate for video)
            // Assuming ~1MB per second for typical video
            let estimated_duration = file_size as f64 / 1_000_000.0;
            
            VideoMetadata {
                path: path.clone(),
                width: 1920,
                height: 1080,
                duration: estimated_duration.max(1.0),
                fps: 30.0,
                total_frames: (estimated_duration.max(1.0) * 30.0) as u64,
                codec: "unknown".to_string(),
            }
        }
        Err(_) => {
            return Err(format!("Cannot read file metadata: {}", path));
        }
    };
    
    Ok(metadata)
}

#[tauri::command]
pub async fn extract_frames(
    path: String,
    roi: ROI,
    options: ExtractOptions,
) -> Result<Vec<Frame>, String> {
    tracing::info!("Extracting frames from: {} with ROI: {:?}", path, roi);
    
    let path_obj = Path::new(&path);
    if !path_obj.exists() {
        return Err(format!("File not found: {}", path));
    }
    
    // Frame extraction requires native video processing library
    // This is a placeholder that returns empty vector
    // Real implementation would use OpenCV or FFmpeg
    tracing::info!("Frame extraction requires native video processing - returning empty frame list");
    
    Ok(vec![])
}

#[tauri::command]
pub async fn detect_scenes(
    path: String,
    threshold: f32,
) -> Result<Vec<u64>, String> {
    tracing::info!("Detecting scenes in: {} with threshold: {}", path, threshold);
    
    let path_obj = Path::new(&path);
    if !path_obj.exists() {
        return Err(format!("File not found: {}", path));
    }
    
    // Scene detection requires native video processing
    // Real implementation would use OpenCV
    tracing::info!("Scene detection requires native video processing - returning empty list");
    
    Ok(vec![])
}
