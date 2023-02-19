use std::str::FromStr;

use crate::{
    handlers::restaurant::RestaurnatError,
    models::restaurant::{Restaurant, TimeSlot},
};

use super::Result;
use actix_web::{
    get,
    web::Path,
    web::{self, Json},
};
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use chrono::NaiveTime;
use uuid::Uuid;

#[get("/{id}")]
pub async fn get(id: Path<Uuid>, db: web::Data<Client>) -> Result<Restaurant> {
    let result = db
        .get_item()
        .table_name("restaurants")
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|e| {
            dbg!(e);
            RestaurnatError::Internal
        })?;
    let mut item = result.item.ok_or(RestaurnatError::Internal)?;
    let mut rbuilder = crate::models::restaurant::RestaurantBuilder::default();
    rbuilder.id(id.into_inner());
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
        if let Some(AttributeValue::N(max_booking_time)) = settings_map.remove("max_booking_time") {
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
    Ok(Json(rbuilder.build()?))
}
