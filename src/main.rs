// Cargo.toml
// [package]
// name = "budget-calculator"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // Cria o roteador
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/calculate", get(calculate_handler))
        .route("/api/budget", post(api_budget_handler));

    // Bind no endereço
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Servidor rodando em http://127.0.0.1:3000");
    
    // Inicia o servidor
    axum::serve(listener, app).await.unwrap();
}

// Estruturas de dados
#[derive(Deserialize)]
struct BudgetQuery {
    income: f64,
}

#[derive(Deserialize)]
struct BudgetRequest {
    income: f64,
}

#[derive(Serialize)]
struct BudgetResponse {
    income: f64,
    needs: BudgetCategory,
    wants: BudgetCategory,
    savings: BudgetCategory,
}

#[derive(Serialize)]
struct BudgetCategory {
    percentage: u8,
    amount: f64,
    description: String,
}

// Função de cálculo
fn calculate_budget(income: f64) -> BudgetResponse {
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

// Handler da página inicial
async fn home_handler() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Regra 50/30/20</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            margin: 0;
            padding: 20px;
        }
        .container {
            background: white;
            padding: 40px;
            border-radius: 15px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            max-width: 500px;
            width: 100%;
        }
        h1 {
            color: #667eea;
            margin: 0 0 10px 0;
        }
        .subtitle {
            color: #666;
            margin-bottom: 30px;
        }
        label {
            display: block;
            color: #333;
            font-weight: bold;
            margin-bottom: 8px;
        }
        input {
            width: 100%;
            padding: 12px;
            font-size: 16px;
            border: 2px solid #ddd;
            border-radius: 8px;
            box-sizing: border-box;
        }
        button {
            width: 100%;
            padding: 14px;
            margin-top: 15px;
            font-size: 16px;
            font-weight: bold;
            color: white;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            border: none;
            border-radius: 8px;
            cursor: pointer;
        }
        button:hover {
            opacity: 0.9;
        }
        .result {
            display: none;
            margin-top: 30px;
        }
        .card {
            padding: 20px;
            margin-bottom: 15px;
            border-radius: 10px;
            border-left: 4px solid;
        }
        .needs { background: #e8f4f8; border-color: #667eea; }
        .wants { background: #fff8e8; border-color: #f59e0b; }
        .savings { background: #e8f8f0; border-color: #10b981; }
        .card h3 {
            margin: 0 0 10px 0;
            font-size: 18px;
        }
        .card .amount {
            font-size: 28px;
            font-weight: bold;
            margin: 10px 0;
        }
        .needs .amount { color: #667eea; }
        .wants .amount { color: #f59e0b; }
        .savings .amount { color: #10b981; }
    </style>
</head>
<body>
    <div class="container">
        <h1>💰 Regra 50/30/20</h1>
        <p class="subtitle">Planeje seu orçamento</p>
        
        <label for="income">Renda Mensal (R$)</label>
        <input type="number" id="income" placeholder="5000.00" step="0.01">
        <button onclick="calculate()">Calcular</button>

        <div class="result" id="result">
            <div class="card needs">
                <h3>🏠 Necessidades (50%)</h3>
                <div class="amount" id="needs"></div>
            </div>
            <div class="card wants">
                <h3>🎮 Desejos (30%)</h3>
                <div class="amount" id="wants"></div>
            </div>
            <div class="card savings">
                <h3>📈 Poupança (20%)</h3>
                <div class="amount" id="savings"></div>
            </div>
        </div>
    </div>

    <script>
        function formatCurrency(value) {
            return 'R$ ' + value.toFixed(2).replace('.', ',');
        }

        async function calculate() {
            const income = parseFloat(document.getElementById('income').value);
            
            if (isNaN(income) || income <= 0) {
                alert('Digite um valor válido!');
                return;
            }

            try {
                const response = await fetch('/calculate?income=' + income);
                const data = await response.json();
                
                document.getElementById('needs').textContent = formatCurrency(data.needs.amount);
                document.getElementById('wants').textContent = formatCurrency(data.wants.amount);
                document.getElementById('savings').textContent = formatCurrency(data.savings.amount);
                
                document.getElementById('result').style.display = 'block';
            } catch (error) {
                alert('Erro: ' + error.message);
            }
        }
    </script>
</body>
</html>
    "#)
}

// Handler GET com query params
async fn calculate_handler(Query(params): Query<BudgetQuery>) -> impl IntoResponse {
    if params.income <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Valor deve ser maior que 0"}))
        ).into_response();
    }

    let budget = calculate_budget(params.income);
    (StatusCode::OK, Json(budget)).into_response()
}

// Handler POST com JSON
async fn api_budget_handler(Json(payload): Json<BudgetRequest>) -> impl IntoResponse {
    if payload.income <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Valor deve ser maior que 0"}))
        ).into_response();
    }

    let budget = calculate_budget(payload.income);
    (StatusCode::OK, Json(budget)).into_response()
}