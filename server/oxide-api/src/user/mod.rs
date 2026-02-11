pub mod auth;

use std::sync::Arc;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use uuid::Uuid;
use crate::AppState;
use crate::dto::user::UserInfo;

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new().route("/me", get(me))
}

#[utoipa::path(
    context_path="/api/v1/users",
    get,
    path="/me",
    tag="user",
    security(
        ("api_jwt_token" = [])
    )
)]
pub async fn me() -> impl IntoResponse {
    (StatusCode::OK, Json(UserInfo{
        id: Uuid::new_v4(),
        first_name: "Денис".to_string(),
        last_name: "Суздальцев".to_string(),
        group_code: "ИТ-21-1".to_string(),
    }))
}