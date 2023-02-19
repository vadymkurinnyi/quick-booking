use std::collections::HashMap;

use crate::{handlers::restaurant::RestaurnatError, models::restaurant::ReservationSettings};

use super::Result;
use actix_web::{
    post,
    web::{self, Json},
};
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use serde::Deserialize;
use uuid::Uuid;

#[post("")]
pub async fn create(
    db: web::Data<Client>,
    restaurant: web::Json<CreateRestaurant>,
) -> Result<Uuid> {
    let id = Uuid::new_v4();
    let CreateRestaurant {
        name,
        address,
        settings,
    } = restaurant.into_inner();
    let request = db
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
        dbg!(e);
        RestaurnatError::Internal
    })?;
    dbg!(result.attributes());
    Ok(Json(id))
}

#[derive(Deserialize)]
pub struct CreateRestaurant {
    pub name: String,
    pub address: String,
    pub settings: ReservationSettings,
}
