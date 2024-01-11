use axum::extract;
use axum::http::header;
use axum::response::IntoResponse;
use axum::response::Json as JsonResponse;
use serde::{Serialize, Deserialize};
use crate::open_ai;
use uuid::Uuid;
use crate::render;

pub async fn generate_notion_cover_image(
    extract::Json(request_body): extract::Json<RequestBody>,
) -> JsonResponse<NotionCoverImage> {
// ) -> impl IntoResponse {

    let generated_image = open_ai::generate_image(request_body.prompt).unwrap();
    let cropped_bytes = crate::image::get_cropped_image(generated_image.url.clone(), 1500, 600);

    let id = Uuid::new_v4();
    let file_name = format!("static/{}.png", id);
    std::fs::write(file_name, cropped_bytes).unwrap();

    let host = if render::is_running_on_render() {
        "https://api.rainsound.ai"
    } else {
        "https://6dd2-2600-1700-1c01-c130-69c0-22ec-7115-9c8d.ngrok-free.app"
    };

    let cropped_image_url = format!("{}/static/{}.png", host, id);

    let notion_cover_image = NotionCoverImage {
        url: cropped_image_url,
    };

    JsonResponse(notion_cover_image)

    // raw image/png bytes
    // let headers = [(header::CONTENT_TYPE, "image/png")];
    // (headers, cropped_bytes)}
}

pub async fn test_generate_notion_cover_image() -> impl IntoResponse {

    let generated_image = open_ai::generate_image("a cute dog".to_string()).unwrap();
    let cropped_bytes = crate::image::get_cropped_image(generated_image.url.clone(), 1500, 600);

    let id = Uuid::new_v4();
    let file_name = format!("static/{}.png", id);
    std::fs::write(file_name, cropped_bytes).unwrap();

    let host = if render::is_running_on_render() {
        "https://api.rainsound.ai"
    } else {
        "https://6dd2-2600-1700-1c01-c130-69c0-22ec-7115-9c8d.ngrok-free.app"
    };

    let cropped_image_url = format!("{}/static/{}.png", host, id);

    let notion_cover_image = NotionCoverImage {
        url: cropped_image_url,
    };

    JsonResponse(notion_cover_image)

    // raw image/png bytes
    // let headers = [(header::CONTENT_TYPE, "image/png")];
    // (headers, cropped_bytes)}
}

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    prompt: String,
}

#[derive(Serialize, Debug)]
pub struct NotionCoverImage {
    url: String,
    // bytes: Vec<u8>,
}
pub type GeneratedImageBytes = Vec<u8>;
