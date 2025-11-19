mod routes;
use crate::routes::app_router;
use dotenvy::dotenv;
use std::env;

mod models;
mod budget;
mod handlers;


#[tokio::main]
async fn main() {
    let app = app_router();
    
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!("🚀 Servidor rodando em http://{}:{}", addr.ip(), addr.port());

    axum::serve(listener, app).await.unwrap();
}

