use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
}

fn get_db_url() -> String {
    env::var("FIREBASE_DB_URL").expect("FIREBASE_DB_URL must be set in .env file")
}

pub async fn get_items_from_firebase(client: &Client) -> Result<Vec<Item>, reqwest::Error> {
    let url = format!("{}/items.json", get_db_url());
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Ok(Vec::new()); // Return empty on error or if no data
    }

    // Firebase returns null if the path doesn't exist, handle that case.
    let text = response.text().await?;
    if text == "null" {
        return Ok(Vec::new());
    }

    let items_map: HashMap<String, Item> = serde_json::from_str(&text).unwrap_or_default();
    let items: Vec<Item> = items_map.into_values().collect();
    Ok(items)
}

pub async fn add_item_to_firebase(client: &Client, item: &Item) -> Result<(), reqwest::Error> {
    let url = format!("{}/items.json", get_db_url());
    client.post(&url).json(item).send().await?;
    Ok(())
}