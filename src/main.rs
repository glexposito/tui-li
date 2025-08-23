use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::Mutex;

mod routes;
mod services;
mod models;

use services::shortener::UrlStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url_store = web::Data::new(Mutex::new(UrlStore::new()));

    println!("🚀 tui-li running at http://127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(url_store.clone())
            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
