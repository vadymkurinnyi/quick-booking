use actix_web::{
    web::{self, Json},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use thiserror::Error;

mod api;
mod protocol;
pub mod repo;
pub mod tables;
type Result<T> = core::result::Result<Json<T>, RestaurnatError>;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/restaurant")
            .service(api::create)
            .service(api::get)
            .service(api::update)
            .service(tables::api::create),
    );
}

use crate::models::restaurant::ReservationSettingsBuilderError;
use crate::models::restaurant::RestaurantBuilderError;
#[derive(Debug, Error, Display)]
pub enum RestaurnatError {
    Internal(Box<dyn std::error::Error>),
    ReservationSettingsBuilderError(#[from] ReservationSettingsBuilderError),
    RestaurantBuilderError(#[from] RestaurantBuilderError),
    ParseIntError(#[from] std::num::ParseIntError),
}

impl ResponseError for RestaurnatError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        dbg!(self);
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json("error")
    }
}
