// axum
use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

async fn home() -> &'static str {
    "Hello, from home page!"
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home)).layer(CorsLayer::permissive());
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
