use uuid::Uuid;

use super::restaurant::TimeSlot;

pub struct Reservation {
    pub id: Uuid,
    pub status: Status,
    pub date: u32,
    pub time: TimeSlot,
    pub table_id: u32,
    pub party_size: u8,
}

pub enum Status {
    Created,
    Confirmed,
    Canceled,
}
