use crate::handlers::{
    get_supplier_by_id, get_suppliers, get_total_spent, get_total_suppliers, get_transaction_by_id, get_transactions, post_supplier, post_transaction
};
use crate::models::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn get_supplier_router() -> axum::Router {
    Router::new()
        .route("/", post(post_supplier).get(get_suppliers))
        .route("/:id", get(get_supplier_by_id))
        .route("/total_suppliers", get(get_total_suppliers))
        .with_state(AppState {
            db: sqlx::SqlitePool::connect_lazy("sqlite://prod.db").unwrap(),
        })
}

pub fn get_transaction_router() -> axum::Router {
    Router::new()
        .route("/", post(post_transaction).get(get_transactions))
        .route("/:id", get(get_transaction_by_id))
        .route("/total_spent", get(get_total_spent))
        .with_state(AppState {
            db: sqlx::SqlitePool::connect_lazy("sqlite://prod.db").unwrap(),
        })
}
