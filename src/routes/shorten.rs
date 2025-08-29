use actix_web::{HttpResponse, Responder, web};
use normalize_url_rs::{Options, OptionsBuilder, normalize_url};
use serde::Deserialize;
use serde_json::json;
use url::Url;

use crate::services::shortener::UrlStore;

fn invalid_url(msg: &'static str) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "error": "invalid_url",
        "message": msg,
        "example": "https://example.com/"
    }))
}

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub long_url: Url,
}

pub async fn shorten_url(
    store: web::Data<std::sync::Mutex<UrlStore>>,
    body: web::Json<ShortenRequest>,
) -> impl Responder {
    // Normalize (url-normalizer 0.2 expects a Url)
    let opts: Options = OptionsBuilder::default()
        .strip_protocol(false)
        .force_http(false)
        .force_https(false)
        .remove_trailing_slash(false)
        .build()
        .expect("options");

    if !matches!(body.long_url.scheme(), "http" | "https") {
        return invalid_url("Only http/https are supported.");
    }

    // normalize and reuse the same error shape
    let normalized = match normalize_url(body.long_url.as_str(), &opts) {
        Ok(s) => s,
        Err(_) => return invalid_url("Provide a valid absolute URL with http/https."),
    };

    let mut store = store.lock().unwrap();
    let mapping = store.add_url(normalized);

    HttpResponse::Ok().json(mapping)
}
