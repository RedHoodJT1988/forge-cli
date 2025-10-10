mod db;

use axum::Router;
use db::{create_db_pool, Item};
use dioxus::prelude::*;
use sqlx::PgPool;
use std::net::SocketAddr;
use tracing::info;
use __PROJECT_NAME__::{app, AddItem, GetItems};

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let db_pool = create_db_pool().await.expect("DB connection failed");
    let app_state = AppState { db_pool };

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
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .0;

    let items = sqlx::query_as!(Item, "SELECT id, name FROM items ORDER BY id")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(items)
}

#[server(AddItem)]
async fn add_item(name: String) -> Result<(), ServerFnError> {
    let state = axum::extract::Extension::<AppState>::from_request_parts(&mut Default::default(), &mut Default::default())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .0;

    sqlx::query!("INSERT INTO items (name) VALUES ($1)", name)
        .execute(&state.db_pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}