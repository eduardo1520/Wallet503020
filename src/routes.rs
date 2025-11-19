use axum::{ routing::{get, post}, Router };

use tower_http::services::ServeDir;
use crate::handlers::{calculate_handler, api_budget_handler};

pub fn app_router() -> Router {
    Router::new()
        .route("/calculate", get(calculate_handler))
        .route("/api/budget", post(api_budget_handler))
        .nest_service("/", ServeDir::new("static").append_index_html_on_directories(true))
}