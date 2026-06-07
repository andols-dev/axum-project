// axum
use axum::{Router, routing::get};

async fn home() -> &'static str {
    "Hello, from home page!"
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
