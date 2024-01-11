use axum::extract;
use axum::http::header;
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::open_ai;

pub async fn generate_notion_cover_image(
    extract::Json(request_body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let generated_image = open_ai::generate_image(request_body.prompt).unwrap();
    let cropped_bytes = crate::image::get_cropped_image(generated_image.url.clone(), 1500, 600);

    // let notion_cover_image = NotionCoverImage {
    //     url: generated_image.url,
    //     bytes: cropped_bytes,
    // };
    // dbg!(&notion_cover_image);
    // JsonResponse(notion_cover_image)

    let headers = [(header::CONTENT_TYPE, "image/png")];
    (headers, cropped_bytes)
}

pub async fn test_generate_notion_cover_image() -> impl IntoResponse {
    let generated_image = open_ai::generate_image("a cute dog".to_string()).unwrap();
    let cropped_bytes = crate::image::get_cropped_image(generated_image.url.clone(), 1500, 600);
    std::fs::write("cropped.png", &cropped_bytes).unwrap();

    // let notion_cover_image = NotionCoverImage {
    //     url: generated_image.url,
    //     bytes: cropped_bytes,
    // };
    // dbg!(&notion_cover_image);
    // JsonResponse(notion_cover_image)

    let headers = [(header::CONTENT_TYPE, "image/png")];
    (headers, cropped_bytes)
}

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    prompt: String,
}

// #[derive(Serialize, Debug)]
// pub struct NotionCoverImage {
//     url: String,
//     bytes: Vec<u8>,
// }
pub type GeneratedImageBytes = Vec<u8>;
