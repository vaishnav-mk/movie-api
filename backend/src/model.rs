use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Media {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub genres: Vec<String>,

    #[validate(range(min = 0.0, max = 5.0))]
    pub rating: f64,

    #[validate(custom = "validate_media_status")]
    pub status: MediaStatus,

    #[serde(rename = "type")]
    pub media_type: MediaType,
}

fn validate_media_status(value: &MediaStatus) -> Result<(), validator::ValidationError> {
    match value {
        MediaStatus::Watching
        | MediaStatus::Watched
        | MediaStatus::Dropped
        | MediaStatus::OnHold
        | MediaStatus::PlanToWatch => Ok(()),
        _ => Err(validator::ValidationError::new("Invalid media status")),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaStatus {
    Watching,
    Watched,
    Dropped,
    OnHold,
    PlanToWatch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaType {
    Movie,
    Show,
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct UpdateMediaSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub genres: Option<Vec<String>>,

    #[validate(range(min = 0.0, max = 5.0))]
    pub rating: Option<f64>,

    #[validate(custom = "validate_media_status")]
    pub status: Option<MediaStatus>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub order: Option<i32>,
}

pub struct AppState {
    pub media_collection: mongodb::Collection<Media>,
    pub start_time: SystemTime,
}

impl AppState {
    pub fn new(media_collection: mongodb::Collection<Media>, start_time: SystemTime) -> Self {
        Self {
            media_collection,
            start_time,
        }
    }
}
