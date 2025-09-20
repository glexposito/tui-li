use serde::Serialize;

#[derive(Serialize)]
pub struct ShortUrlResponse {
    pub short_url: String,
    pub long_url: String,
}
