use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handler::handler, state::state::AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthz", get(handler::health_checker_handler))
        .route(
            "/api/project/:project/release/:release",
            get(handler::get_report_handler),
        )
        .with_state(app_state)
}
