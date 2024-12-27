use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
mod api;
mod models;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| App::new().route("/generate", web::post().to(api::handlers::generate)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
