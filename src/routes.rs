use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{home::home, login::login, profile::profile, register::register},
    state::AppState,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/api/home", get(home))
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/profile", get(profile))
}
