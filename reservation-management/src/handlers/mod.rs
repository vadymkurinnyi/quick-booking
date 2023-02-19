use actix_web::web;

pub mod restaurant;

pub fn add_handlers(cfg: &mut web::ServiceConfig) {
    restaurant::configure(cfg);
}
