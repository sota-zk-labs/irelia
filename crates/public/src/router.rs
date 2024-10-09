use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Router,
};

// use rust_core::ports::question::QuestionPort;
use crate::controllers::app_state::AppState;
use crate::controllers::job::add_job;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest(
            "/v1/gateway",
            Router::new()
                .route("/add_job", post(add_job))
                .with_state(app_state),
        )
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
