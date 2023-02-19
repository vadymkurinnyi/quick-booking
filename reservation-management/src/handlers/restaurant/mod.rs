use actix_web::{
    web::{self, Json},
    HttpResponse, ResponseError,
};
use derive_more::Display;
use thiserror::Error;

mod create;
type Result<T> = core::result::Result<Json<T>, RestaurnatError>;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/restaurant").service(create::create));
}

#[derive(Debug, Error, Display)]
pub enum RestaurnatError {
    Internal,
}

impl ResponseError for RestaurnatError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json("error")
    }
}
