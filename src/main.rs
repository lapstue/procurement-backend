mod models;

use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;

use crate::models::{
    AppState, SupplierDbRow, SupplierLines, SupplierResponse, TransactionDbRow, TransactionLines,
    TransactionResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://prod.db")
        .await?;

    // Suppliers table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS suppliers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            Supplier TEXT NOT NULL,
            SupplierNameOriginal TEXT NOT NULL,
            SupplierCountry TEXT NOT NULL,
            VatID TEXT NOT NULL,
            NACE TEXT NOT NULL
        );
        "#,
    )
    .execute(&db)
    .await?;

    // Transactions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            InvoiceNumber TEXT NOT NULL,
            InvoiceDate TEXT,
            DueDate TEXT,
            TransactionValueNOK REAL NOT NULL,
            SpendCategoryL1 TEXT NOT NULL,
            SpendCategoryL2 TEXT NOT NULL,
            SpendCategoryL3 TEXT NOT NULL,
            SpendCategoryL4 TEXT NOT NULL
        );
        "#,
    )
    .execute(&db)
    .await?;

    let app_state = models::AppState { db };

    let app = Router::new()
        .route("/suppliers", post(post_supplier).get(get_suppliers))
        .route("/suppliers/:id", get(get_supplier_by_id))
        .route(
            "/transactions",
            post(post_transaction).get(get_transactions),
        )
        .route("/transactions/:id", get(get_transaction_by_id))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

// --- Suppliers ---

async fn post_supplier(
    State(state): State<AppState>,
    Json(payload): Json<SupplierLines>,
) -> Result<Json<SupplierResponse>, axum::http::StatusCode> {
    let result = sqlx::query(
        r#"
        INSERT INTO suppliers (Supplier, SupplierNameOriginal, SupplierCountry, VatID, NACE)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(&payload.Supplier)
    .bind(&payload.SupplierNameOriginal)
    .bind(&payload.SupplierCountry)
    .bind(&payload.VatID)
    .bind(&payload.NACE)
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.last_insert_rowid();

    Ok(Json(SupplierResponse {
        id,
        Supplier: payload.Supplier,
        SupplierNameOriginal: payload.SupplierNameOriginal,
        SupplierCountry: payload.SupplierCountry,
        VatID: payload.VatID,
        NACE: payload.NACE,
    }))
}

async fn get_suppliers(
    State(state): State<AppState>,
) -> Result<Json<Vec<SupplierResponse>>, axum::http::StatusCode> {
    let rows: Vec<SupplierDbRow> = sqlx::query_as(
        "SELECT id, Supplier, SupplierNameOriginal, SupplierCountry, VatID, NACE FROM suppliers",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

async fn get_supplier_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<SupplierResponse>, axum::http::StatusCode> {
    let row: SupplierDbRow = sqlx::query_as(
        "SELECT id, Supplier, SupplierNameOriginal, SupplierCountry, VatID, NACE FROM suppliers WHERE id = ?1"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(row.into()))
}

// --- Transactions ---

async fn post_transaction(
    State(state): State<AppState>,
    Json(payload): Json<TransactionLines>,
) -> Result<Json<TransactionResponse>, axum::http::StatusCode> {
    let result = sqlx::query(
        r#"
        INSERT INTO transactions
        (InvoiceNumber, InvoiceDate, DueDate, TransactionValueNOK,
         SpendCategoryL1, SpendCategoryL2, SpendCategoryL3, SpendCategoryL4)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        "#,
    )
    .bind(&payload.InvoiceNumber)
    .bind(&payload.InvoiceDate.map(|dt| dt.to_rfc3339()))
    .bind(&payload.DueDate.map(|dt| dt.to_rfc3339()))
    .bind(payload.TransactionValueNOK)
    .bind(&payload.SpendCategoryL1)
    .bind(&payload.SpendCategoryL2)
    .bind(&payload.SpendCategoryL3)
    .bind(&payload.SpendCategoryL4)
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.last_insert_rowid();

    let db_row = TransactionDbRow {
        id,
        InvoiceNumber: payload.InvoiceNumber,
        InvoiceDate: payload.InvoiceDate.map(|dt| dt.to_rfc3339()),
        DueDate: payload.DueDate.map(|dt| dt.to_rfc3339()),
        TransactionValueNOK: payload.TransactionValueNOK,
        SpendCategoryL1: payload.SpendCategoryL1,
        SpendCategoryL2: payload.SpendCategoryL2,
        SpendCategoryL3: payload.SpendCategoryL3,
        SpendCategoryL4: payload.SpendCategoryL4,
    };

    Ok(Json(db_row.into_response()))
}

async fn get_transactions(
    State(state): State<AppState>,
) -> Result<Json<Vec<TransactionResponse>>, axum::http::StatusCode> {
    let rows: Vec<TransactionDbRow> = sqlx::query_as(
        "SELECT id, InvoiceNumber, InvoiceDate, DueDate, TransactionValueNOK, SpendCategoryL1, SpendCategoryL2, SpendCategoryL3, SpendCategoryL4 FROM transactions"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows.into_iter().map(|r| r.into_response()).collect()))
}

async fn get_transaction_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<TransactionResponse>, axum::http::StatusCode> {
    let row: TransactionDbRow = sqlx::query_as(
        "SELECT id, InvoiceNumber, InvoiceDate, DueDate, TransactionValueNOK, SpendCategoryL1, SpendCategoryL2, SpendCategoryL3, SpendCategoryL4 FROM transactions WHERE id = ?1"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(row.into_response()))
}
