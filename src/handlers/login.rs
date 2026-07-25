use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use serde::{Deserialize, Serialize};

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
    (
        StatusCode::OK,
        Json(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
        }),
    )
}
