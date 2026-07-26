use axum::{extract::State, http::StatusCode, response::IntoResponse};

use tower_cookies::{Cookie, Cookies};

use crate::state::AppState;

pub async fn logout(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    println!("LOGOUT CALLED");
    let cookie = cookies.get("session");

    if let Some(cookie) = cookie {
        let token = cookie.value();

        sqlx::query("DELETE FROM sessions WHERE token = ?")
            .bind(token)
            .execute(&state.db)
            .await
            .unwrap();
    }
    if let Some(cookie) = cookies.get("session") {
        println!("Token: {}", cookie.value());
    } else {
        println!("No session cookie found");
    }
    cookies.remove(Cookie::build("session").path("/").into());

    (StatusCode::OK, "Logged out")
}
