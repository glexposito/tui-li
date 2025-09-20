use crate::services::shortener_service::ShortenerService;
use actix_web::{HttpResponse, Responder, web};
use anyhow::Error;
use serde_json::json;

pub async fn redirect_url(
    service: web::Data<ShortenerService>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();

    match service.resolve(&id).await {
        Ok(Some(url)) => HttpResponse::Found()
            .append_header(("Location", url))
            .finish(),

        Ok(None) => HttpResponse::NotFound().body("URL not found"),

        Err(e) => {
            // also log it
            tracing::error!("shorten failed: {:?}", e);
            HttpResponse::InternalServerError().json(error_payload(&e))
        }
    }
}

fn error_payload(e: &Error) -> serde_json::Value {
    let causes: Vec<String> = e.chain().map(|c| c.to_string()).collect();
    // Debug shows backtrace if enabled (RUST_BACKTRACE=1 and debug symbols)
    let backtrace = format!("{:?}", e.backtrace());

    json!({
        "error": "shorten_failed",
        "message": e.to_string(),
        "causes": causes,            // full cause chain (top first)
        "backtrace": backtrace,      // may say "disabled" if not enabled
    })
}
