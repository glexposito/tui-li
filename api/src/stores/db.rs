use aws_config;
use aws_sdk_dynamodb::Client;

pub async fn make_ddb_client() -> Client {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if let Ok(endpoint) = std::env::var("DYNAMODB_ENDPOINT") {
        loader = loader.endpoint_url(endpoint);
    }

    let config = loader.load().await;
    Client::new(&config)
}
