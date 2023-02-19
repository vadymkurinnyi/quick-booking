use uuid::Uuid;

pub struct Table {
    pub id: Uuid,
    pub name: String,
    pub capacity: u8,
}
