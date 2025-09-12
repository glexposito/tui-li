use crate::models::app_config::AppConfig;
use aws_config;
use aws_sdk_dynamodb::Client;

pub async fn make_ddb_client(config: &AppConfig) -> Client {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());
    loader = loader.endpoint_url(&config.dynamodb_endpoint);

    let aws_config = loader.load().await;
    Client::new(&aws_config)
}
