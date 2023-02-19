use std::error::Error;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{
    model::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
    Client,
};

pub async fn create() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    client
}

static TABLE_NAME: &str = "restaurants";
pub async fn ensure_created(client: &Client) -> Result<(), Box<dyn Error>> {
    let tables = client.list_tables().send().await?;
    if let Some(names) = tables.table_names() {
        if names.iter().any(|name| name == TABLE_NAME) {
            return Ok(());
        }
    }
    let key = "id";
    let pk = AttributeDefinition::builder()
        .attribute_name(key)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(key)
        .key_type(KeyType::Hash)
        .build();

    client
        .create_table()
        .table_name(String::from(TABLE_NAME))
        .key_schema(ks)
        .attribute_definitions(pk)
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await?;
    Ok(())
}
