mod db;

use axum::{extract::State, routing::{get, post}, Form, Router};
use db::{connect_to_mongodb, Item};
use futures::stream::TryStreamExt;
use maud::{html, Markup, DOCTYPE};
use mongodb::{bson::doc, Collection, Database};
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::info;

#[derive(Clone)]
struct AppState {
    db: Database,
}

#[derive(Deserialize)]
struct NewItem {
    name: String,
}

async fn root_page(State(state): State<AppState>) -> Markup {
    let collection: Collection<Item> = state.db.collection("items");
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut items = Vec::new();
    while let Ok(Some(item)) = cursor.try_next().await {
        items.push(item);
    }

    html! {
        (DOCTYPE)
        html {
            head {
                title { "Trestle + HTMX + MongoDB" }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                script src="https://unpkg.com/htmx.org@1.9.12" {}
                script src="https://cdn.tailwindcss.com" {}
            }
            body class="bg-gray-800 text-white flex justify-center py-12" {
                main class="container mx-auto p-8 text-center border border-gray-600 rounded-lg shadow-xl bg-gray-900 w-1/2" {
                    h1 class="text-5xl font-bold mb-4" { "Todo List 📝" }
                    p class="text-xs mt-4 mb-8 text-gray-500" { "Connects to the 'items' collection in your MongoDB database." }

                    div id="item-list" {
                        (render_items(items))
                    }

                    form class="mt-8" hx-post="/items" hx-target="#item-list" hx-swap="outerHTML" hx-on::after-request="this.reset()" {
                        input type="text" name="name" class="bg-gray-700 text-white p-3 rounded-l-lg" placeholder="New todo item...";
                        button type="submit" class="bg-blue-600 hover:bg-blue-700 text-white font-bold p-3 rounded-r-lg" { "Add Item" }
                    }
                }
            }
        }
    }
}

async fn add_item(State(state): State<AppState>, Form(item): Form<NewItem>) -> Markup {
    let collection: Collection<Item> = state.db.collection("items");
    let new_doc = Item { name: item.name };
    collection.insert_one(new_doc, None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut items = Vec::new();
    while let Ok(Some(item)) = cursor.try_next().await {
        items.push(item);
    }

    render_items(items)
}

fn render_items(items: Vec<Item>) -> Markup {
    html! {
        div id="item-list" {
            @if items.is_empty() {
                p class="text-gray-400" { "No items in the database yet." }
            } @else {
                ul class="list-none text-left" {
                    @for item in items {
                        li class="bg-gray-800 p-3 my-2 rounded-lg" { (item.name) }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let db = connect_to_mongodb().await.expect("Failed to connect to MongoDB.");
    let app_state = AppState { db };

    let app = Router::new()
        .route("/", get(root_page))
        .route("/items", post(add_item))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}