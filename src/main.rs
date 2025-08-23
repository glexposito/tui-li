use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

mod models;
mod routes;
mod services;

use services::shortener::UrlStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url_store = web::Data::new(Mutex::new(UrlStore::new()));

    println!("🚀 tui-li running at http://127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(url_store.clone())
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
