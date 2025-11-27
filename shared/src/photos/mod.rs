use chrono::Utc;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewPhoto {
    pub timestamp: i64,
    pub plant_id: Uuid,
    pub photo_binary: Vec<u8>,
}

impl NewPhoto {
    pub fn new(image: DynamicImage, plant_id: Uuid) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            plant_id,
            photo_binary: image.as_bytes().iter().cloned().collect(),
        }
    }
}
