use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDetectionConfig {
    pub threshold: f32,
    pub min_scene_length: u32,
    pub frame_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneChange {
    pub frame_index: u64,
    pub timestamp: f64,
    pub similarity: f32,
}

#[tauri::command]
pub async fn detect_scenes(
    video_path: String,
    config: SceneDetectionConfig,
) -> Result<Vec<SceneChange>, String> {
    tracing::info!("Detecting scenes in: {} with threshold: {}", video_path, config.threshold);
    
    let path = Path::new(&video_path);
    if !path.exists() {
        return Err(format!("File not found: {}", video_path));
    }
    
    // Scene detection requires native video processing library
    // Real implementation would use OpenCV for frame comparison
    // For now, return empty list - user can implement with their preferred library
    tracing::info!("Scene detection requires native video processing library");
    
    Ok(vec![])
}

#[tauri::command]
pub async fn calculate_frame_similarity(
    frame1_data: Vec<u8>,
    frame2_data: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<f32, String> {
    // Validate input
    if frame1_data.len() != frame2_data.len() {
        return Err("Frame data length mismatch".to_string());
    }
    
    if frame1_data.is_empty() {
        return Err("Frame data is empty".to_string());
    }
    
    // Calculate histogram-based similarity
    let sample_count = (frame1_data.len() / 4).min(1000);
    if sample_count == 0 {
        return Ok(1.0);
    }
    
    let step = (frame1_data.len() / 4) / sample_count;
    let mut total_diff = 0f32;
    
    for i in 0..sample_count {
        let idx = i * step * 4;
        if idx + 3 >= frame1_data.len() {
            break;
        }
        
        let r1 = frame1_data[idx] as f32;
        let g1 = frame1_data[idx + 1] as f32;
        let b1 = frame1_data[idx + 2] as f32;
        
        let r2 = frame2_data[idx] as f32;
        let g2 = frame2_data[idx + 1] as f32;
        let b2 = frame2_data[idx + 2] as f32;
        
        let diff = ((r1 - r2).powi(2) + (g1 - g2).powi(2) + (b1 - b2).powi(2)).sqrt();
        total_diff += diff;
    }
    
    let avg_diff = total_diff / sample_count as f32;
    let similarity = 1.0 - (avg_diff / 441.67).min(1.0); // Max RGB distance
    
    Ok(similarity)
}

#[tauri::command]
pub fn get_video_info(path: String) -> Result<serde_json::Value, String> {
    let path_obj = Path::new(&path);
    
    if !path_obj.exists() {
        return Err(format!("File not found: {:?}", path_obj));
    }
    
    let metadata = std::fs::metadata(&path).map_err(|e| format!("Cannot read file: {}", e))?;
    
    Ok(serde_json::json!({
        "exists": true,
        "is_file": metadata.is_file(),
        "is_dir": metadata.is_dir(),
        "size": metadata.len(),
        "name": path_obj.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown"),
        "extension": path_obj.extension()
            .and_then(|e| e.to_str())
            .unwrap_or(""),
    }))
}
