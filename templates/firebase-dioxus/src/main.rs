mod firebase;

use axum::Router;
use dioxus::prelude::*;
use firebase::{add_item_to_firebase, get_items_from_firebase, Item};
use reqwest::Client;
use std::net::SocketAddr;
use tracing::info;
use __PROJECT_NAME__::{app, AddItem, GetItems};

#[derive(Clone)]
struct AppState {
    http_client: Client,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let app_state = AppState {
        http_client: Client::new(),
    };

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

    get_items_from_firebase(&state.http_client).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(AddItem)]
async fn add_item(name: String) -> Result<(), ServerFnError> {
    let state = axum::extract::Extension::<AppState>::from_request_parts(&mut Default::default(), &mut Default::default())
        .await.map_err(|e| ServerFnError::ServerError(e.to_string()))?.0;

    let new_item = Item { name };
    add_item_to_firebase(&state.http_client, &new_item).await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}