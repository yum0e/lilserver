use axum::{response::IntoResponse, Json};

#[tracing::instrument(name = "Health check", skip_all)]
pub async fn check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}
