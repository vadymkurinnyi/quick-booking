use std::sync::Arc;

use actix_web::web::{self, Data};
use aws_sdk_dynamodb::Client;

pub mod dynamo_db;

pub struct Services {
    client: Data<Client>,
}
impl Services {
    pub fn get_client(&self) -> Data<Client> {
        self.client.clone()
    }
}
pub fn add_services(cfg: &mut web::ServiceConfig, state: Arc<Services>) {
    cfg.app_data(state.get_client());
}

pub async fn configure_services() -> Arc<Services> {
    let dynamo_db = dynamo_db::create().await;
    dynamo_db::ensure_created(&dynamo_db)
        .await
        .expect("Db not set");
    let client = Data::new(dynamo_db);
    Arc::new(Services { client })
}
