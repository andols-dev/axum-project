use rand::{RngCore, thread_rng};
use sqlx::{Row, SqlitePool};

pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];

    thread_rng().fill_bytes(&mut bytes);

    hex::encode(bytes)
}

pub async fn get_user_id_from_token(db: &SqlitePool, token: &str) -> Option<i64> {
    let result = sqlx::query("SELECT user_id FROM sessions WHERE token = ?")
        .bind(token)
        .fetch_optional(db)
        .await
        .ok()?;

    result.map(|row| row.get("user_id"))
}
