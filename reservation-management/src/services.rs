use std::sync::Arc;

use actix_web::web::{self, Data};
use aws_sdk_dynamodb::Client;

use crate::restaurant::repo::{RestaurantRepo, RestaurantRepoDynamoDb};

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
    let restaurant_repo: Box<dyn RestaurantRepo> =
        Box::new(RestaurantRepoDynamoDb::new(state.get_client()));

    cfg.app_data(Data::new(restaurant_repo));
}

pub async fn configure_services() -> Arc<Services> {
    let dynamo_db = dynamo_db::create().await;
    dynamo_db::ensure_created(&dynamo_db)
        .await
        .expect("Db not set");
    let client = Data::new(dynamo_db);

    Arc::new(Services { client })
}
