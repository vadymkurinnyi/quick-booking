use actix_web::web;

pub mod api;
pub mod repo;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").service(api::create));
}
