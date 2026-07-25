use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

pub async fn register(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    if payload.email.is_empty() {
        return (StatusCode::BAD_REQUEST, "Email missing".to_string());
    }

    if payload.password.is_empty() {
        return (StatusCode::BAD_REQUEST, "Password missing".to_string());
    }
    println!("Register request:");
    println!("First name: {}", payload.first_name);
    println!("Last name: {}", payload.last_name);
    println!("Email: {}", payload.email);

    (StatusCode::OK, format!("Register {}", payload.email))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
