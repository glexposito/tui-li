use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::shortener::UrlStore;

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub long_url: String,
}

pub async fn shorten_url(
    store: web::Data<std::sync::Mutex<UrlStore>>,
    body: web::Json<ShortenRequest>,
) -> impl Responder {
    let mut store = store.lock().unwrap();
    let mapping = store.add_url(body.long_url.clone());

    HttpResponse::Ok().json(mapping)
}
