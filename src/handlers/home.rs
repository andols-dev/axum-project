use axum::Json;
use serde_json::{json, Value};
pub async fn home() -> Json<Value> {
    Json(json!({
        "message": "Hej från Axum!"
    }))
}

