use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

pub fn generate_image(original_prompt: String) -> Result<GeneratedImage, String> {
    let client = Client::new();
    let api_key = std::env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY not set");
    let auth_header_value = format!("Bearer {}", api_key);

    let body = RequestBody::new(original_prompt.clone());
    let serialized_body = serde_json::to_string(&body).expect("Failed to serialize body");

    let raw_response = client
        .post("https://api.openai.com/v1/images/generations")
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_header_value).expect("Invalid Authorization header value"),
        )
        .body(serialized_body)
        .send()
        .expect("Failed to send request");

    if raw_response.status().is_success() {
        let response = raw_response
            .json::<Response>()
            .expect("Failed to parse response");

        let generated_image = GeneratedImage {
            url: response.data[0].url.clone(),
            original_prompt,
            revised_prompt: response.data[0].revised_prompt.clone(),
        };

        return Ok(generated_image);
    }

    Err("Failed to generate image.".to_string())
}

#[derive(Deserialize, Debug)]
pub struct Response {
    data: Vec<ResponseDataItem>,
}

#[derive(Deserialize, Debug)]
struct ResponseDataItem {
    revised_prompt: String,
    url: String,
}

pub struct GeneratedImage {
    pub url: String,
    pub original_prompt: String,
    pub revised_prompt: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    prompt: String,
    n: u8,
    size: String,
}

impl RequestBody {
    fn new(prompt: String) -> Self {
        Self {
            model: "dall-e-3".to_string(),
            prompt,
            n: 1,
            size: "1792x1024".to_string(),
        }
    }
}
