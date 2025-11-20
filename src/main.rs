mod routes;
use crate::routes::app_router;

mod models;
mod budget;
mod handlers;
mod server;


#[tokio::main]
async fn main() {
    let app = app_router();
    server::start_server(app).await;
}

