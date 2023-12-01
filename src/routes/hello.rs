use axum::response::IntoResponse;

#[tracing::instrument(name = "Hello World", skip_all)]
pub async fn world() -> impl IntoResponse {
    "Hello, World!"
}
