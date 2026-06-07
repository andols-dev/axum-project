use tower_http::cors::CorsLayer;

mod handlers;
mod routes;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    let db = sqlx::SqlitePool::connect("sqlite://db/db.sqlite")
        .await
        .expect("Failed to connect to database");
    println!("{:?}", std::env::current_dir().unwrap());
    let app_state = AppState { db };
    let app = routes::create_router()
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
