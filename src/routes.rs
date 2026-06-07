use axum::{Router, routing::get};

use crate::{handlers::home::home, state::AppState};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", get(home))
}
