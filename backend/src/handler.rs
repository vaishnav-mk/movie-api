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

#[get("/media/{id}")]
pub async fn get_media_by_id(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id_str = path.into_inner();
    let media_id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid media ID");
        }
    };

    let query = doc! { "_id": media_id.to_string() };
    let media_result = data.media_collection.find_one(query, None).await;

    match media_result {
        Ok(Some(media)) => HttpResponse::Ok().json(media),
        Ok(None) => HttpResponse::NotFound().body("Media not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching media"),
    }
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

#[delete("/media/{id}")]
async fn delete_media_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let media_collection = &data.media_collection;

    let id = path.into_inner();
    let media_id = match Uuid::parse_str(&id) {
        Ok(id) => id.to_string(),
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid media ID");
        }
    };

    let query = doc! { "_id": media_id };
    let result = media_collection
        .delete_one(query, None)
        .await
        .expect("Error deleting media");

    if result.deleted_count == 0 {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Media with ID: {} not found", &id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let success_response = GenericResponse {
        status: "success".to_string(),
        message: format!("Media with ID: {} has been deleted", &id),
    };
    HttpResponse::Ok().json(success_response)
}

#[patch("/media/{id}")]
async fn edit_media_handler(
    path: web::Path<String>,
    body: web::Json<UpdateMediaSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let media_collection = &data.media_collection;

    let id = path.into_inner();
    let media_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid media ID");
        }
    };

    let query = doc! { "_id": media_id.to_string() };
    let mut update = doc! { "$set": {} };

    if let Some(title) = &body.title {
        update.insert("$set", doc! { "title": title });
    }
    if let Some(description) = &body.description {
        update.insert("$set", doc! { "description": description });
    }
    if let Some(genres) = &body.genres {
        update.insert("$set", doc! { "genres": genres });
    }
    if let Some(rating) = body.rating {
        update.insert("$set", doc! { "rating": rating });
    }

    if let Some(status) = &body.status {
        let status_bson = match status {
            MediaStatus::Watching => Bson::String("Watching".to_string()),
            MediaStatus::Watched => Bson::String("Watched".to_string()),
            MediaStatus::Dropped => Bson::String("Dropped".to_string()),
            MediaStatus::OnHold => Bson::String("OnHold".to_string()),
            MediaStatus::PlanToWatch => Bson::String("PlanToWatch".to_string()),
        };
        update.insert("$set", doc! { "status": status_bson });
    }

    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    let result = media_collection
        .find_one_and_update(query, update, options)
        .await
        .expect("Error updating media");

    if result.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Media with ID: {} not found", &id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let media = result.expect("Error unwrapping updated media");
    let json_response = SingleMediaResponse {
        status: "success".to_string(),
        data: MediaData { media },
    };

    HttpResponse::Ok().json(json_response)
}

#[get("/generate-media/{n}")]
pub async fn generate_random_media(
    path: web::Path<u32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let n = path.into_inner();
    let media_collection = &data.media_collection;

    let mut rng = rand::thread_rng();
    let mut media_items: Vec<Media> = Vec::new();

    for _ in 0..n {
        let id = Uuid::new_v4();
        let title = format!("Media #{}", rng.gen::<u32>());
        let description = format!("Random description for Media #{}", rng.gen::<u32>());
        let genres: Vec<String> = (0..3)
            .map(|_| {
                let random_genre = ["Action", "Adventure", "Comedy", "Drama", "Sci-Fi"];
                let index = rand::thread_rng().gen_range(0..random_genre.len());
                random_genre[index].to_string()
            })
            .collect();

        let rating = rng.gen_range(0.0..5.0);
        let status = MediaStatus::PlanToWatch;
        let media_type = if rand::random::<bool>() {
            MediaType::Movie
        } else {
            MediaType::Show
        };

        let media = Media {
            id: Some(id.to_string()),
            title,
            description,
            genres,
            rating,
            status,
            media_type,
        };

        let _result = media_collection
            .insert_one(media.clone(), None)
            .await
            .expect("Error inserting media");

        media_items.push(media);
    }

    let json_response = MediaListResponse {
        status: "success".to_string(),
        results: media_items.len(),
        media: media_items,
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(media_list_handler)
        .service(get_media_by_id)
        .service(create_media_handler)
        .service(delete_media_handler)
        .service(edit_media_handler)
        .service(generate_random_media);

    conf.service(scope);
}
