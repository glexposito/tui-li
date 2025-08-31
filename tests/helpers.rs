#![allow(dead_code)]
use actix_web::{App, test, web};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_dynamodb::{Client, config::Credentials, types::AttributeValue as Av};
use testcontainers::ContainerAsync;
use testcontainers_modules::{dynamodb_local, testcontainers::runners::AsyncRunner};
use tui_li::routes;
use tui_li::services::shortener_service::ShortenerService;
use tui_li::stores::db::ensure_table;
use tui_li::stores::url_store::UrlStore;

/// Keeps DynamoDB Local alive for the whole test (drops -> container stops)
pub struct DdbGuard {
    _container: ContainerAsync<dynamodb_local::DynamoDb>,
}

/// Boot a fresh DynamoDB Local + client, ensure table, and build your service.
/// Returns the service (as `web::Data<_>`) + a guard that holds the container.

pub async fn setup_service() -> (web::Data<ShortenerService>, DdbGuard) {
    let (shortener_service, _client, guard) = setup_service_with_client().await;
    (shortener_service, guard)
}

pub async fn setup_service_with_client() -> (web::Data<ShortenerService>, Client, DdbGuard) {
    let container = dynamodb_local::DynamoDb::default().start().await.unwrap();

    let host = container.get_host().await.expect("host");
    let port = container.get_host_port_ipv4(8000).await.expect("port");
    let endpoint = format!("http://{host}:{port}");

    let cfg = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(endpoint)
        .region(Region::new("us-east-1"))
        .credentials_provider(Credentials::for_tests())
        .load()
        .await;

    let client = Client::new(&cfg);

    let table = "tui-li-urls";
    ensure_table(&client, table).await.expect("ensure_table");

    let store = UrlStore::new(client.clone(), table.to_string());
    let service = ShortenerService::new(store);
    let shortener_service = web::Data::new(service);

    (
        shortener_service,
        client,
        DdbGuard {
            _container: container,
        },
    )
}

pub async fn init_app(
    service_data: web::Data<ShortenerService>,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    test::init_service(App::new().app_data(service_data).configure(routes::config)).await
}

pub async fn seed_url(
    client: &Client,
    table: &str,
    id: &str,
    long_url: &str,
    created_at_rfc3339: &str,
    url_hash_hex: &str, // hardcoded hash
) -> Result<(), aws_sdk_dynamodb::Error> {
    // ID#...
    client
        .put_item()
        .table_name(table)
        .item("pk", Av::S(format!("ID#{id}")))
        .item("id", Av::S(id.to_string()))
        .item("long_url", Av::S(long_url.to_string()))
        .item("created_at", Av::S(created_at_rfc3339.to_string()))
        // drop this line if you want to overwrite in re-seeds:
        .condition_expression("attribute_not_exists(pk)")
        .send()
        .await?;

    // URL#<hash>
    client
        .put_item()
        .table_name(table)
        .item("pk", Av::S(format!("URL#{url_hash_hex}")))
        .item("id", Av::S(id.to_string()))
        .item("long_url", Av::S(long_url.to_string()))
        .item("created_at", Av::S(created_at_rfc3339.to_string()))
        .condition_expression("attribute_not_exists(pk)")
        .send()
        .await?;

    Ok(())
}
