use axum::{extract::State, http::StatusCode, response::IntoResponse};
use tower_cookies::Cookies;

use crate::{state::AppState, utils::session::get_user_id_from_token};

pub async fn profile(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let token = match cookies.get("session") {
        Some(cookie) => cookie.value().to_string(),

        None => {
            return (StatusCode::UNAUTHORIZED, "No session".to_string());
        }
    };

    let user_id = get_user_id_from_token(&state.db, &token).await;

    match user_id {
        Some(id) => (StatusCode::OK, format!("Logged in as user {}", id)),

        None => (StatusCode::UNAUTHORIZED, "Invalid session".to_string()),
    }
}
