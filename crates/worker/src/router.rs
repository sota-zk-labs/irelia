use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(root))
        .fallback(handler_404)
}

async fn root() -> &'static str {
    "Server is running!"
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
