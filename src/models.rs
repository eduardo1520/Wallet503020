// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BudgetQuery {
    pub income: f64,
}

#[derive(Deserialize)]
pub struct BudgetRequest {
    pub income: f64,
}

#[derive(Serialize)]
pub struct BudgetResponse {
    pub income: f64,
    pub needs: BudgetCategory,
    pub wants: BudgetCategory,
    pub savings: BudgetCategory,
}

#[derive(Serialize)]
pub struct BudgetCategory {
    pub percentage: u8,
    pub amount: f64,
    pub description: String,
}