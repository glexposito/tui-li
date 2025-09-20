use crate::models::app_settings::AppSettings;
use aws_config;
use aws_sdk_dynamodb::Client;

pub async fn make_ddb_client(settings: &AppSettings) -> Client {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if !settings.dynamodb_endpoint.is_empty() {
        loader = loader.endpoint_url(&settings.dynamodb_endpoint);
    }

    let aws_config = loader.load().await;
    Client::new(&aws_config)
}
