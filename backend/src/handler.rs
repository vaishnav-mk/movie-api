use crate::{
    model::AppState,
    response::{GenericResponse, MediaData, MediaListResponse, SingleMediaResponse},
};
use actix_web::{get, web, HttpResponse, Responder};
use chrono::prelude::*;

#[get("/health")]
async fn health_checker_handler(data: web::Data<AppState>) -> impl Responder {
    const MESSAGE: &str = "Server is running";

    let uptime = match data.start_time.elapsed() {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    };

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: format!("{} since {} seconds", MESSAGE, uptime),
    };

    HttpResponse::Ok().json(response_json)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(health_checker_handler));
}
