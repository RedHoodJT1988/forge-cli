mod db;

use axum::Router;
use db::{connect_to_mongodb, Item};
use dioxus::prelude::*;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use std::net::SocketAddr;
use tracing::info;
use __PROJECT_NAME__::{app, AddItem, GetItems};

#[derive(Clone)]
struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let db = connect_to_mongodb().await.expect("DB connection failed");
    let app_state = AppState { db };

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .register_server_fns("/api")
                .serve_dioxus_application(ServeConfig::builder().build(), move || {
                    VirtualDom::new(app)
                })
                .with_state(app_state)
                .into_make_service(),
        )
        .await
        .unwrap();
}

#[server(GetItems)]
async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    let state = axum::extract::Extension::<AppState>::from_request_parts(&mut Default::default(), &mut Default::default())
        .await.map_err(|e| ServerFnError::ServerError(e.to_string()))?.0;

    let collection: Collection<Item> = state.db.collection("items");
    let mut cursor = collection.find(None, None).await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let mut items = Vec::new();
    while let Ok(Some(item)) = cursor.try_next().await {
        items.push(item);
    }
    Ok(items)
}

#[server(AddItem)]
async fn add_item(name: String) -> Result<(), ServerFnError> {
    let state = axum::extract::Extension::<AppState>::from_request_parts(&mut Default::default(), &mut Default::default())
        .await.map_err(|e| ServerFnError::ServerError(e.to_string()))?.0;

    let collection: Collection<Item> = state.db.collection("items");
    let new_doc = Item { name };
    collection.insert_one(new_doc, None).await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}