use std::{collections::HashMap, error::Error};

use actix_web::web::Data;
use async_trait::async_trait;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use json_patch::Patch;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::table::Table;

#[async_trait]
pub trait TableRepo {
    async fn create(&self, rest_id: Uuid, table: CreateTable) -> Result<Uuid, Box<dyn Error>>;
    async fn get(&self, rest_id: Uuid, id: Uuid) -> Result<Table, Box<dyn Error>>;
    async fn update(&self, rest_id: Uuid, id: Uuid, pth: Patch) -> Result<(), Box<dyn Error>>;
}
#[derive(Deserialize)]
pub struct CreateTable {
    pub name: String,
    pub capacity: u8,
}

pub struct TableRepoDynamoDb {
    client: Data<Client>,
}

impl TableRepoDynamoDb {
    pub fn new(client: Data<Client>) -> Self {
        Self { client }
    }
}
#[async_trait]
impl TableRepo for TableRepoDynamoDb {
    async fn create(&self, rest_id: Uuid, table: CreateTable) -> Result<Uuid, Box<dyn Error>> {
        let id = Uuid::new_v4();
        let table = AttributeValue::M(HashMap::from([
            ("id".to_string(), AttributeValue::S(id.to_string())),
            ("name".to_string(), AttributeValue::S(table.name)),
            (
                "capacity".to_string(),
                AttributeValue::N(table.capacity.to_string()),
            ),
        ]));
        let request = self
            .client
            .update_item()
            .table_name("restaurants")
            .key("id", AttributeValue::S(rest_id.to_string()))
            .update_expression(
                "set #tables = list_append(if_not_exists(#tables, :empty_list), :table)",
            )
            .expression_attribute_names("#tables", "tables")
            .expression_attribute_values(":table", AttributeValue::L(vec![table]))
            .expression_attribute_values(":empty_list", AttributeValue::L(vec![]));
        request.send().await.map_err(|e| {
            dbg!(&e);
            e
        })?;
        Ok(id)
    }
    async fn get(&self, rest_id: Uuid, id: Uuid) -> Result<Table, Box<dyn Error>> {
        todo!()
    }
    async fn update(&self, rest_id: Uuid, id: Uuid, pth: Patch) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
