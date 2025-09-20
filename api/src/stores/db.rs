use crate::models::app_settings::AppSettings;
use aws_config;
use aws_sdk_dynamodb::Client;

pub async fn make_ddb_client(config: &AppSettings) -> Client {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if !config.dynamodb_endpoint.is_empty() {
        loader = loader.endpoint_url(&config.dynamodb_endpoint);
    }

    let aws_config = loader.load().await;
    Client::new(&aws_config)
}
