use actix_web::{web, HttpResponse, Responder};
use crate::services::shortener::UrlStore;

pub async fn redirect_url(
    store: web::Data<std::sync::Mutex<UrlStore>>,
    path: web::Path<String>,
) -> impl Responder {
    let store = store.lock().unwrap();
    let id = path.into_inner();

    if let Some(url) = store.get_url(&id) {
        HttpResponse::Found()
            .append_header(("Location", url))
            .finish()
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}
