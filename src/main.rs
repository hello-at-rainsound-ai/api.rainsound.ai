use routes::*;
use axum::Router;
use axum::routing::{get, post};

mod routes;
mod open_ai;
mod image;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate-notion-cover-image", post(generate_notion_cover_image))
        .route("/healthz", get(health_check))
        .fallback(not_found);

    // Run our app with hyper, listening globally on the specified port.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host_and_port = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(host_and_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() {}
