use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Item {
    pub name: String,
}

pub async fn connect_to_mongodb() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env file");
    let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set in .env file");

    let mut client_options = ClientOptions::parse(mongo_uri).await?;
    client_options.app_name = Some("forge-app".to_string());

    let client = Client::with_options(client_options)?;
    Ok(client.database(&db_name))
}