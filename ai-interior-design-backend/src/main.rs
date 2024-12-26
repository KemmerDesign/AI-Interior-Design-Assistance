use actix_web::{web, App, HttpServer};

mod api;
mod services;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate", web::post().to(api::handlers::generate))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}