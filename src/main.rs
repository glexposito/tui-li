use actix_web::{App, HttpServer, web};
use tui_li::routes;
use tui_li::services::shortener_service::ShortenerService;
use tui_li::stores::db::{make_ddb_client};
use tui_li::stores::url_store::UrlStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // build DynamoDB client and services
    let table = "tui-li-urls";
    let client = make_ddb_client().await;
    let store = UrlStore::new(client, table.to_string());
    let service = ShortenerService::new(store);

    // wrap in Arc<Data>
    let service_data = web::Data::new(service);

    // env config
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    println!("🚀 tui-li running at http://{host}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(service_data.clone())
            .configure(routes::config)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
