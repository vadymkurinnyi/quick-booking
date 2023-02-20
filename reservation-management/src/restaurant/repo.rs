use std::{collections::HashMap, error::Error, str::FromStr};

use crate::models::restaurant::{Restaurant, TimeSlot};
use actix_web::web::Data;
use async_trait::async_trait;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use chrono::NaiveTime;
use json_patch::Patch;
use uuid::Uuid;

use super::protocol::CreateRestaurant;

#[async_trait]
pub trait RestaurantRepo {
    async fn create(&self, restaurant: CreateRestaurant) -> Result<Uuid, Box<dyn Error>>;
    async fn get(&self, id: Uuid) -> Result<Restaurant, Box<dyn Error>>;
    async fn update(&self, id: Uuid, pth: Patch) -> Result<(), Box<dyn Error>>;
}

pub struct RestaurantRepoDynamoDb {
    client: Data<Client>,
}

impl RestaurantRepoDynamoDb {
    pub fn new(client: Data<Client>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RestaurantRepo for RestaurantRepoDynamoDb {
    async fn create(&self, restaurant: CreateRestaurant) -> Result<Uuid, Box<dyn Error>> {
        let id = Uuid::new_v4();
        let CreateRestaurant {
            name,
            address,
            settings,
        } = restaurant;
        let request = self
            .client
            .put_item()
            .table_name("restaurants")
            .item("id", AttributeValue::S(id.to_string()))
            .item("name", AttributeValue::S(name))
            .item("address", AttributeValue::S(address))
            .item(
                "settings",
                AttributeValue::M(HashMap::from([
                    (
                        "max_days".to_string(),
                        AttributeValue::N(settings.max_days.to_string()),
                    ),
                    (
                        "max_booking_time".to_string(),
                        AttributeValue::N(settings.max_booking_time.to_string()),
                    ),
                    (
                        "time_slots".to_string(),
                        AttributeValue::Ss(
                            settings
                                .time_slots
                                .iter()
                                .map(|t| t.time.to_string())
                                .collect::<Vec<String>>(),
                        ),
                    ),
                ])),
            );
        let result = request.send().await.map_err(|e| {
            dbg!(&e);
            e
        })?;
        Ok(id)
    }
    async fn get(&self, id: Uuid) -> Result<Restaurant, Box<dyn Error>> {
        let result = self
            .client
            .get_item()
            .table_name("restaurants")
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| {
                dbg!(&e);
                e
            })?;
        let mut item = result.item.ok_or(anyhow::anyhow!("item not found"))?;
        let mut rbuilder = crate::models::restaurant::RestaurantBuilder::default();
        rbuilder.id(id);
        if let Some(AttributeValue::S(name)) = item.remove("name") {
            rbuilder.name(name);
        }
        if let Some(AttributeValue::S(address)) = item.remove("address") {
            rbuilder.address(address);
        }
        if let Some(AttributeValue::M(settings_map)) = item.remove("settings") {
            let mut settings_map = settings_map;
            let mut s_builder = crate::models::restaurant::ReservationSettingsBuilder::default();
            if let Some(AttributeValue::N(max_days)) = settings_map.remove("max_days") {
                s_builder.max_days(max_days.parse()?);
            }
            if let Some(AttributeValue::N(max_booking_time)) =
                settings_map.remove("max_booking_time")
            {
                s_builder.max_booking_time(max_booking_time.parse()?);
            }

            if let Some(AttributeValue::Ss(time_slots)) = settings_map.remove("time_slots") {
                let time_slots = time_slots
                    .into_iter()
                    .filter_map(|s| {
                        let time = NaiveTime::from_str(&s).ok()?;
                        Some(TimeSlot { time })
                    })
                    .collect();
                s_builder.time_slots(time_slots);
            }
            rbuilder.settings(s_builder.build()?);
        }
        Ok(rbuilder.build()?)
    }
    async fn update(&self, id: Uuid, pth: Patch) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
