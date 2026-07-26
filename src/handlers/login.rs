use std::mem::transmute;

use crate::utils::session::generate_session_token;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use serde::{Deserialize, Serialize};
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};

use crate::state::AppState;
use crate::utils::password::verify_password;
use sqlx::Row;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query("SELECT id, password_hash FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if user.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                success: false,
                message: "Invalid email or password".to_string(),
            }),
        );
    }

    let user = user.unwrap();

    let user_id: i64 = user.get("id");

    let password_hash: String = user.get("password_hash");

    if !verify_password(&payload.password, &password_hash) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                success: false,
                message: "Invalid email or password".to_string(),
            }),
        );
    }
    let token = generate_session_token();

    sqlx::query("INSERT INTO sessions (user_id, token) VALUES (?, ?)")
        .bind(user_id)
        .bind(&token)
        .execute(&state.db)
        .await
        .unwrap();

    cookies.add(
        Cookie::build(("session", token))
            .http_only(true)
            .same_site(SameSite::Lax)
            .path("/")
            .into(),
    );

    (
        StatusCode::OK,
        Json(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        }),
    )
}
