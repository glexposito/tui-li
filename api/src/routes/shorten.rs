use crate::models::app_settings::AppSettings;
use crate::models::short_url_response::ShortUrlResponse;
use crate::services::shortener_service::ShortenerService;
use actix_web::{HttpResponse, Responder, web};
use normalize_url_rs::{Options, OptionsBuilder, normalize_url};
use serde::Deserialize;
use serde_json::json;
use url::Url;

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
    service: web::Data<ShortenerService>,
    settings: web::Data<AppSettings>,
    body: web::Json<ShortenRequest>,
) -> impl Responder {
    if !matches!(body.long_url.scheme(), "http" | "https") {
        return invalid_url("Only http/https are supported.");
    }

    // normalize and reuse the same error shape
    let opts: Options = OptionsBuilder::default()
        .strip_protocol(false)
        .force_http(false)
        .force_https(false)
        .build()
        .expect("options");

    let normalized = match normalize_url(body.long_url.as_str(), &opts) {
        Ok(s) => s,
        Err(_) => return invalid_url("Provide a valid absolute URL with http/https."),
    };

    match service.shorten(&normalized).await {
        Ok(id) => HttpResponse::Ok().json(ShortUrlResponse {
            short_url: format!("{}{}", settings.short_url_base, id),
            long_url: normalized,
        }),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "shorten_failed",
            "message": e.to_string(),
        })),
    }
}
