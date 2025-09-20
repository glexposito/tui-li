use actix_web::{App, HttpServer, web, HttpResponse};
use actix_web::middleware::Logger;
use tui_li::models::app_config::AppConfig;
use tui_li::routes;
use tui_li::services::shortener_service::ShortenerService;
use tui_li::stores::db::make_ddb_client;
use tui_li::stores::url_store::UrlStore;
use tracing_subscriber::{EnvFilter, fmt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logs
    fn init_logs() {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info,actix_web=info,tui_li=debug,aws_sdk_dynamodb=info".parse().unwrap());
        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .compact()
            .init();
    }

    // in main() before building HttpServer:
    init_logs();

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
            // Access log (method path status bytes time UA)
            .wrap(Logger::new(r#"%r %s %b %T "%{User-Agent}i""#))
            // 200/OK health for Nginx/Lightsail
            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
            // Make JSON extractor errors obvious (avoid generic "error dispatching")
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                tracing::warn!("json extractor failed: {err}");
                actix_web::error::ErrorBadRequest("invalid json")
            }))
            .app_data(service_data.clone())
            .app_data(config_data.clone())
            .configure(routes::config)
    })
    .bind((host, port))?
    .run()
    .await
}
