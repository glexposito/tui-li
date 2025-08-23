use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

mod models;
mod routes;
mod services;

use services::shortener::UrlStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url_store = web::Data::new(Mutex::new(UrlStore::new()));

    // Read HOST/PORT from env (defaults keep your current local behavior)
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    println!("🚀 tui-li running at http://{host}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(url_store.clone())
            .configure(routes::config)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
