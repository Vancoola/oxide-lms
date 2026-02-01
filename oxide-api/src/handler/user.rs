use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use crate::dto::user::UserInfo;

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