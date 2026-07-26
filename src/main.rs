use axum::http::{HeaderValue, Method};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

mod handlers;
mod routes;
mod state;
mod utils;

use state::AppState;

#[tokio::main]
async fn main() {
    let db = sqlx::SqlitePool::connect("sqlite://db/db.sqlite")
        .await
        .expect("Failed to connect to database");
    println!("{:?}", std::env::current_dir().unwrap());
    let app_state = AppState { db };
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);
    let app = routes::create_router()
        .with_state(app_state)
        .layer(cors)
        .layer(CookieManagerLayer::new());

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
