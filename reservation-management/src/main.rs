use actix_web::{App, HttpServer};
use anyhow::Ok;

mod handlers;
pub mod models;
mod restaurant;
mod services;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().expect("load .env");
    env_logger::init();
    let app_state = services::configure_services().await;

    Ok(HttpServer::new(move || {
        App::new()
            .configure(restaurant::configure)
            .configure(|cfg| services::add_services(cfg, app_state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?)
}
