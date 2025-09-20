use crate::services::shortener_service::ShortenerService;
use actix_web::{HttpResponse, Responder, web};
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

        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": "shorten_failed",
            "message": e.to_string(),
        })),
    }
}
