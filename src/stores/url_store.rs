use anyhow::Result;
use aws_sdk_dynamodb::types::{Put, TransactWriteItem};
use aws_sdk_dynamodb::{Client, types::AttributeValue as Av};
// import the SDK v1 adapters:
use crate::models::url_item::UrlItem;
use serde_dynamo::aws_sdk_dynamodb_1::{from_item, to_item};

pub struct UrlStore {
    client: Client,
    table: String,
}

impl UrlStore {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            table: "url".to_string(),
        }
    }

    pub async fn get_by_pk(&self, pk: &str) -> Result<Option<UrlItem>> {
        let out = self
            .client
            .get_item()
            .table_name(&self.table)
            .key("pk", Av::S(pk.to_string()))
            .send()
            .await?;

        match out.item() {
            Some(item) => {
                let model: UrlItem = from_item(item.clone())?;
                Ok(Some(model))
            }
            None => Ok(None),
        }
    }

    pub async fn create_many<I>(&self, items: I) -> Result<()>
    where
        I: IntoIterator<Item = UrlItem>,
    {
        // Build TransactWriteItem vec
        let mut transact_items = Vec::new();
        for it in items {
            let map = to_item(it)?;
            let put = Put::builder()
                .table_name(&self.table)
                .set_item(Some(map))
                .condition_expression("attribute_not_exists(pk)")
                .build()?;
            transact_items.push(TransactWriteItem::builder().put(put).build());
        }

        // Enforce the 25-item limit for a single atomic transaction
        if transact_items.len() > 25 {
            // You can either error or chunk. Error keeps atomicity guarantees clear.
            anyhow::bail!(
                "Too many items for a single DynamoDB transaction (max 25). Got {}",
                transact_items.len()
            );
        }

        self.client
            .transact_write_items()
            .set_transact_items(Some(transact_items))
            .send()
            .await?;

        Ok(())
    }
}
