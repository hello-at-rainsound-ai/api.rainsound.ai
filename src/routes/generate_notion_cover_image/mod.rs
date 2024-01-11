use axum::extract;
use axum::Json as JsonResponse;
use serde::{Deserialize, Serialize};
use crate::open_ai;

pub async fn generate_notion_cover_image(
    extract::Json(request_body): extract::Json<RequestBody>,
) -> JsonResponse<NotionCoverImage> {
    let generated_image = open_ai::generate_image(request_body.prompt).unwrap();
    let notion_cover_image = NotionCoverImage {
        url: generated_image.url,
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
}
