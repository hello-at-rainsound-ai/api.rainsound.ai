use crate::extensions::*;
use routes::*;
use tower_http::services::ServeDir;
use axum::{extract::Request, routing::get, Router};

mod extensions;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_request)) // The wildcard "/*anthing" syntax doesn't match the root route, so we have to register that one separately.
        .route("/*anything", get(handle_request))
        .route("/healthz", get(health_check));

    // Run our app with hyper, listening globally on the specified port.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host_and_port = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(host_and_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// For now, all of our routes return HTML.
async fn handle_request(req: Request) -> axum::response::Html<String> {
    let route = Route::from_request(&req);
    route.html().into_axum_html_response()
}

async fn health_check() {}
