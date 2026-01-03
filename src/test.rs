use axum::{Json, response::IntoResponse};
use rusqlite::{Connection, Result};

pub async fn test(x: u8) -> impl IntoResponse {
    let y = abc().await;
    let r = serde_json::json!([
        {
            "test": "OK",
        }
    ]);
    Json(r)
}
pub async fn abc() -> String {
    "hello".to_string()
}
