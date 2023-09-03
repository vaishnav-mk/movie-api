use crate::model::{AppState, Media};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use mongodb::{Client, Collection};
use std::time::SystemTime;

mod handler;
mod model;
mod response;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let port = 8001;

    let mongo_client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to MongoDB");

    let db_name = "media_api";

    let db = mongo_client.database(db_name);

    let media_collection: Collection<Media> = db.collection("media");

    let start_time = SystemTime::now();

    let app_data = web::Data::new(AppState::new(media_collection, start_time));

    println!("🚀 Starting server at: http://localhost:{}", port);

    HttpServer::new(move || {
        let app = App::new();

        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .supports_credentials();

        app.app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
    .map_err(|e| {
        println!("Server failed to start: {}", e);
        e
    })
}
