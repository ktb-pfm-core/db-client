use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::{handler::handler, state::state::AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(handler::health_checker_handler))
        .route(
            "/api/v1/report",
            post(handler::get_report_handler),
        )
        .with_state(app_state)
}
