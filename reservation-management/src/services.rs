use actix_web::web::{self, Data};
use aws_sdk_dynamodb::Client;

pub mod dynamo_db;

pub fn add_services(cfg: &mut web::ServiceConfig, client: Data<Client>) {
    cfg.app_data(client);
}
