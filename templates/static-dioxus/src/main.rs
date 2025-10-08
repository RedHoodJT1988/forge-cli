use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;
use __PROJECT_NAME__::app;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

  // The router that will serve your static assets
  let app = Router::new().nest_service("/", ServeDir::new("dist"));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  info!("🚀 Server listening on http://{}", addr);

  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}