use crate::models::{BudgetCategory, BudgetResponse};

pub trait BudgetStrategy {
    fn allocate(&self, income: f64) -> BudgetResponse;
}

enum Category {
    Needs,
    Wants,
    Savings,
}

impl Category {
    fn percentage(&self) -> u8 {
        match self {
            Category::Needs => 50,
            Category::Wants => 30,
            Category::Savings => 20,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Category::Needs => "Necessidades essenciais",
            Category::Wants => "Desejos e lazer",
            Category::Savings => "Poupança e investimentos",
        }
    }
}

pub struct FiftyThirtyTwenty;

impl FiftyThirtyTwenty {
    fn build(&self, c: Category, income: f64) -> BudgetCategory {
        let p = c.percentage();
        BudgetCategory {
            percentage: p,
            amount: income * (p as f64 / 100.0),
            description: c.description().to_string(),
        }
    }
}

impl BudgetStrategy for FiftyThirtyTwenty {
    fn allocate(&self, income: f64) -> BudgetResponse {
        BudgetResponse {
            income,
            needs: self.build(Category::Needs, income),
            wants: self.build(Category::Wants, income),
            savings: self.build(Category::Savings, income),
        }
    }
}

pub fn calculate_budget(income: f64) -> BudgetResponse {
    FiftyThirtyTwenty.allocate(income)
}