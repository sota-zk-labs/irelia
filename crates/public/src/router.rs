use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Router,
};

use crate::app_state::AppState;
use crate::controllers::job::{get_proof, get_status};
use crate::controllers::worker_job::add_worker_job;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .nest(
            "/v1/gateway",
            Router::new()
                .route("/add_job", post(add_worker_job))
                .route("/get_status", get(get_status))
                .route("/get_proof", get(get_proof))
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
