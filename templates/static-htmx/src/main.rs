use axum::{response::Html, routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use std::net::SocketAddr;
use tracing::info;
use tower_http::services::ServeDir;

async fn hello_page() -> Markup {
  html! {
    (DOCTYPE)
    html {
      head {
        title { "My HTMX App" }
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        script src="https://unpkg.com/htmx.org@1.9.12" {}
        script src="https://cdn.tailwindcss.com" {}
      }
      body class="bg-gray-900 text-white flex items-center justify-center h-screen" {
        div class="text-center" {
          h1 class="text-5xl font-bold mb-4" { "Hello from Forge! 🚀"}
          p class="text-xl mb-8" { "This page was rendered by the Axum backend using Maud."}
          button
            class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            hx-get="/clicked"
            hx-swap="outerHTML"
          {
            "Click Me!"
          }
        }
      }
    }
  }
}

async fn clicked_handler() -> Markup {
  html! {
    div class="text-center" {
      p class="text-green-400 text-2xl" { "HTMX is working!" }
    }
  }
}

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  let app = Router::new()
    .route("/", get(hello_page))
    .route("/clicked", get(clicked_handler));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  info!("🚀 Server listening on http://{}", addr);

  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}