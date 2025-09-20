use actix_web::{App, HttpServer, web};
use tui_li::models::app_settings::AppSettings;
use tui_li::routes;
use tui_li::services::shortener_service::ShortenerService;
use tui_li::stores::db::make_ddb_client;
use tui_li::stores::url_store::UrlStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load settings
    let settings = AppSettings::from_env_or_default();

    // build DynamoDB client and services
    let table = "tui-li-urls";
    let client = make_ddb_client(&settings).await;
    let store = UrlStore::new(client, table.to_string());
    let service = ShortenerService::new(store);

    println!(
        "🚀 tui-li running at http://{}:{}",
        settings.host, settings.port
    );

    // capture for bind before wrapping in Data
    let host = settings.host.clone();
    let port = settings.port;

    // wrap once; clone Data in the factory
    let service_data = web::Data::new(service);
    let settings_data = web::Data::new(settings);

    HttpServer::new(move || {
        App::new()
            .app_data(service_data.clone())
            .app_data(settings_data.clone())
            .configure(routes::config)
    })
    .bind((host, port))?
    .run()
    .await
}
