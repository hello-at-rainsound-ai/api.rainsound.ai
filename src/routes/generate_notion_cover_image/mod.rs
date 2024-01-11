use axum::extract;
use axum::Json as JsonResponse;
use serde::{Deserialize, Serialize};
use crate::open_ai;

pub async fn generate_notion_cover_image(
    extract::Json(request_body): extract::Json<RequestBody>,
) -> JsonResponse<NotionCoverImage> {
    let generated_image = open_ai::generate_image(request_body.prompt).unwrap();
    let cropped_bytes = crate::image::get_cropped_image(generated_image.url.clone(), 1500, 600);
    std::fs::write("cropped.png", &cropped_bytes).unwrap();

    let notion_cover_image = NotionCoverImage {
        url: generated_image.url,
        bytes: cropped_bytes,
    };
    dbg!(&notion_cover_image);
    JsonResponse(notion_cover_image)
}

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    prompt: String,
}

#[derive(Serialize, Debug)]
pub struct NotionCoverImage {
    url: String,
    bytes: Vec<u8>,
}
