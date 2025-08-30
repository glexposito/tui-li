use actix_web::{App, HttpServer, web};
use tui_li::routes;
use tui_li::services::shortener_service::ShortenerService;
use tui_li::stores::db::{ensure_table, make_ddb_client};
use tui_li::stores::url_store::UrlStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // build DynamoDB client
    let client = make_ddb_client().await;

    if let Err(e) = ensure_table(&client, "url").await {
        eprintln!("⚠️ failed to ensure table in local mode: {e:?}");
    } else {
        println!("📦 ensured table `url` for local DynamoDB");
    }

    let store = UrlStore::new(client);
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
