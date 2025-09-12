use actix_web::{App, HttpServer, web};
use tui_li::models::app_config::AppConfig;
use tui_li::routes;
use tui_li::services::shortener_service::ShortenerService;
use tui_li::stores::db::make_ddb_client;
use tui_li::stores::url_store::UrlStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load config
    let config = AppConfig::from_env_or_default();

    // build DynamoDB client and services
    let table = "tui-li-urls";
    let client = make_ddb_client(&config).await;
    let store = UrlStore::new(client, table.to_string());
    let service = ShortenerService::new(store);

    println!(
        "🚀 tui-li running at http://{}:{}",
        config.host, config.port
    );

    // capture for bind before wrapping in Data
    let host = config.host.clone();
    let port = config.port;

    // wrap once; clone Data in the factory
    let service_data = web::Data::new(service);
    let config_data = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .app_data(service_data.clone())
            .app_data(config_data.clone())
            .configure(routes::config)
    })
    .bind((host, port))?
    .run()
    .await
}
