use actix_web::{
    get, patch, post,
    web::{self, Json, Path},
};
use uuid::Uuid;

use crate::restaurant::{tables::repo::CreateTable, RestaurnatError, Result};

use super::repo::TableRepo;
type Repo = web::Data<Box<dyn TableRepo>>;

#[post("{id}")]
pub async fn create(db: Repo, rest_id: Path<Uuid>, table: web::Json<CreateTable>) -> Result<Uuid> {
    let id = db
        .create(rest_id.into_inner(), table.into_inner())
        .await
        .map_err(RestaurnatError::Internal)?;
    Ok(Json(id))
}
