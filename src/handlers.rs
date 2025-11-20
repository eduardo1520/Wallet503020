use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

use crate::budget::calculate_budget;
use crate::models::{BudgetQuery, BudgetRequest};

fn invalid_income_response() -> Response {
    (StatusCode::BAD_REQUEST,Json(serde_json::json!({"error": "Valor deve ser maior que 0"}))).into_response()
}

pub async fn calculate_handler(Query(params): Query<BudgetQuery>) -> impl IntoResponse {
    if params.income <= 0.0 {
        return invalid_income_response();
    }

    let budget = calculate_budget(params.income);
    (StatusCode::OK, Json(budget)).into_response()
}

pub async fn api_budget_handler(Json(payload): Json<BudgetRequest>) -> impl IntoResponse {
    if payload.income <= 0.0 {
        return invalid_income_response();
    }

    let budget = calculate_budget(payload.income);
    (StatusCode::OK, Json(budget)).into_response()
}