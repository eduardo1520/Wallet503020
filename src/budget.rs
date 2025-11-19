use crate::models::{BudgetCategory, BudgetResponse};

pub fn calculate_budget(income: f64) -> BudgetResponse {
    BudgetResponse {
        income,
        needs: BudgetCategory {
            percentage: 50,
            amount: income * 0.5,
            description: "Necessidades essenciais".to_string(),
        },
        wants: BudgetCategory {
            percentage: 30,
            amount: income * 0.3,
            description: "Desejos e lazer".to_string(),
        },
        savings: BudgetCategory {
            percentage: 20,
            amount: income * 0.2,
            description: "Poupança e investimentos".to_string(),
        },
    }
}