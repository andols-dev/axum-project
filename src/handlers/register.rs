use axum::Json;
use serde::Deserialize;

pub async fn register(Json(payload): Json<RegisterRequest>) -> Result<String, String> {
    if payload.email.is_empty() {
        return Err("Email missing".to_string());
    }

    if payload.password.is_empty() {
        return Err("password missing".to_string());
    }

    Ok(format!("Register {}", payload.email))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}
