use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Builder)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub settings: ReservationSettings,
}

#[derive(Deserialize, Serialize, Clone, Builder)]
pub struct ReservationSettings {
    pub max_days: u32,
    pub max_booking_time: u32,
    pub time_slots: Vec<TimeSlot>,
}

use chrono::NaiveTime;
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct TimeSlot {
    pub time: NaiveTime,
}
