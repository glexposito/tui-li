use anyhow::Result;
use aws_config;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};

pub async fn make_ddb_client() -> Client {
    let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if let Ok(endpoint) = std::env::var("DYNAMODB_ENDPOINT") {
        loader = loader.endpoint_url(endpoint);
    }

    let config = loader.load().await;
    Client::new(&config)
}

pub async fn ensure_table(client: &Client, table_name: &str) -> Result<()> {
    let tables = client.list_tables().send().await?;
    if tables.table_names().contains(&table_name.to_string()) {
        return Ok(()); // already exists
    }

    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()?,
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build()?,
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()?,
        )
        .send()
        .await?;

    Ok(())
}
