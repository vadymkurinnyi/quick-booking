use actix_web::{web::Data, App, HttpServer};
use anyhow::Ok;
use services::dynamo_db;

mod handlers;
pub mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().expect("load .env");
    env_logger::init();

    let dynamo_db = dynamo_db::create().await;
    dynamo_db::ensure_created(&dynamo_db)
        .await
        .expect("Db not set");
    let client = Data::new(dynamo_db);

    let _ = HttpServer::new(move || {
        App::new()
            .configure(handlers::add_handlers)
            .configure(|cfg| services::add_services(cfg, client.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
