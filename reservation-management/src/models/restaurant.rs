use serde::Deserialize;
use uuid::Uuid;

pub struct Resaurant {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub settings: ReservationSettings,
}

#[derive(Deserialize)]
pub struct ReservationSettings {
    pub max_days: u32,
    pub max_booking_time: u32,
    pub time_slots: Vec<TimeSlot>,
}

use chrono::NaiveTime;
#[derive(PartialEq, Eq, Deserialize)]
pub struct TimeSlot {
    pub time: NaiveTime,
}
