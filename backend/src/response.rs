use serde::Serialize;

use crate::model::Media;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct MediaData {
    pub media: Media,
}

#[derive(Serialize, Debug)]
pub struct SingleMediaResponse {
    pub status: String,
    pub data: MediaData,
}

#[derive(Serialize, Debug)]
pub struct MediaListResponse {
    pub status: String,
    pub results: usize,
    pub media: Vec<Media>,
}
