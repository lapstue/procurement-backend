mod handlers;
mod models;
mod routers;
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://prod.db")
        .await?;

    let app = Router::new()
        .nest("/suppliers", routers::get_supplier_router())
        .nest("/transactions", routers::get_transaction_router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Running on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
