use serde::Deserialize;

use crate::models::restaurant::ReservationSettings;

#[derive(Deserialize)]
pub struct CreateRestaurant {
    pub name: String,
    pub address: String,
    pub settings: ReservationSettings,
}
