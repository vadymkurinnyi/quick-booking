use crate::{
    models::restaurant::Restaurant,
    restaurant::{protocol::CreateRestaurant, RestaurnatError},
};

use super::{repo::RestaurantRepo, Result};
use actix_web::{
    get, patch, post,
    web::{self, Json, Path},
};
use json_patch::Patch;
use uuid::Uuid;
type Repo = web::Data<Box<dyn RestaurantRepo>>;

#[post("")]
pub async fn create(db: Repo, restaurant: web::Json<CreateRestaurant>) -> Result<Uuid> {
    let id = db
        .create(restaurant.into_inner())
        .await
        .map_err(RestaurnatError::Internal)?;
    Ok(Json(id))
}

#[get("/{id}")]
pub async fn get(id: Path<Uuid>, db: Repo) -> Result<Restaurant> {
    let restaurant = db
        .get(id.into_inner())
        .await
        .map_err(RestaurnatError::Internal)?;
    Ok(Json(restaurant))
}

#[patch("/{id}")]
pub async fn update(id: Path<Uuid>, pth: web::Json<Patch>, db: Repo) -> Result<()> {
    db.update(id.into_inner(), pth.into_inner())
        .await
        .map_err(RestaurnatError::Internal)?;
    Ok(Json(()))
}
