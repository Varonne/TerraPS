use axum::{http::HeaderMap, Json};
use uuid::Uuid;

use crate::models::account::AccountLogin;

pub async fn login(header: HeaderMap) -> Json<AccountLogin> {
    let uid = header.get("Uid").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());

    Json(AccountLogin::new(uid))
}

pub async fn sync_data() {}
