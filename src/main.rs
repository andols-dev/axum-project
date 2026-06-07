// axum
use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

mod state;
use state::AppState;
    
async fn home() -> &'static str {
    "Hello, from home page!"
}
#[tokio::main]
async fn main() {
    let db = sqlx::SqlitePool::connect("sqlite:./db.sqlite")
        .await
        .unwrap();
    let app_state = AppState { db };
    let app = Router::new()
        .route("/", get(home))
        .with_state(app_state)
        .layer(CorsLayer::permissive());
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
