mod handlers;
mod models;
mod routers;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .nest("/suppliers", routers::get_supplier_router())
        .nest("/transactions", routers::get_transaction_router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Running on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
