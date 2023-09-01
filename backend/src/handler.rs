use crate::{
    model::{AppState, Media, MediaStatus, MediaType, QueryOptions, UpdateMediaSchema},
    response::{GenericResponse, MediaData, MediaListResponse, SingleMediaResponse},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use bson::Bson;
use futures_util::stream::StreamExt;
use mongodb::bson::doc;
use rand::prelude::*;
use uuid::Uuid;

#[get("/health")]
async fn health_checker_handler(data: web::Data<AppState>) -> impl Responder {
    let start_time = data.start_time;
    let uptime = match start_time.elapsed() {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    };

    const MESSAGE: &str = "Server is running";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: format!("{} (Uptime: {} seconds)", MESSAGE, uptime),
    };

    HttpResponse::Ok().json(response_json)
}

#[get("/media")]
pub async fn media_list_handler(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let media_collection = &data.media_collection;

    let limit = opts.limit.unwrap_or(10);

    let query = doc! {};

    let options = mongodb::options::FindOptions::builder()
        .limit(limit as i64)
        .build();

    let mut cursor = media_collection
        .find(query, options)
        .await
        .expect("Error fetching media");

    let mut media_list: Vec<Media> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => media_list.push(document),
            Err(err) => {
                eprintln!("Error fetching media: {}", err);
            }
        }
    }

    if media_list.is_empty() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: "No media found".to_string(),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let json_response = MediaListResponse {
        status: "success".to_string(),
        results: media_list.len(),
        media: media_list,
    };

    HttpResponse::Ok().json(json_response)
}

#[post("/media")]
async fn create_media_handler(body: web::Json<Media>, data: web::Data<AppState>) -> impl Responder {
    let media_collection = &data.media_collection;
    let mut media = body.into_inner();

    let id = Uuid::new_v4();
    media.id = Some(id.to_string());

    let existing_media = media_collection
        .find_one(doc! {"title": &media.title}, None)
        .await
        .expect("Error checking for existing media");

    if existing_media.is_some() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Media with title: '{}' already exists", &media.title),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let _result = media_collection
        .insert_one(media.clone(), None)
        .await
        .expect("Error inserting media");

    let json_response = SingleMediaResponse {
        status: "success".to_string(),
        data: MediaData { media },
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(media_list_handler)
        .service(create_media_handler);

    conf.service(scope);
}
