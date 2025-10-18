use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{FromRow, MySqlPool};
use std::env;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Item {
  pub id: i32,
  pub name: String,
}

pub async fn create_db_pool() -> Result<MySqlPool, sqlx::Error> {
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
  MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&db_url)
    .await
}