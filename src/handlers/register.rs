use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::state::AppState;
use crate::utils::password::hash_password;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> String {
    let password_hash = hash_password(&payload.password);
    sqlx::query(
        r#"
        INSERT INTO users (
            first_name,
            last_name,
            email,
            password_hash
        )
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.email)
    .bind(&password_hash)
    .execute(&state.db)
    .await
    .unwrap();

    "User saved".to_string()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
