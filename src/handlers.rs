// --- Suppliers ---

use axum::{
    Json,
    extract::{Path, State},
};

use crate::models::{
    AppState, SupplierDbRow, SupplierLines, SupplierResponse, TransactionDbRow, TransactionLines,
    TransactionResponse,
};

pub async fn post_supplier(
    State(state): State<AppState>,
    Json(payload): Json<SupplierLines>,
) -> Result<Json<SupplierResponse>, axum::http::StatusCode> {
    // Use `sqlx::query!` for compile-time checking of the SQL query
    let result = sqlx::query!(
        r#"
        INSERT INTO suppliers (Supplier, SupplierNameOriginal, SupplierCountry, VatID, NACE)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        payload.Supplier,
        payload.SupplierNameOriginal,
        payload.SupplierCountry,
        payload.VatID,
        payload.NACE,
    )
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

pub async fn get_suppliers(
    State(state): State<AppState>,
) -> Result<Json<Vec<SupplierResponse>>, axum::http::StatusCode> {
    // Use `sqlx::query_as!` for compile-time checking and automatic deserialization
    let rows: Vec<SupplierDbRow> = sqlx::query_as!(
        SupplierDbRow,
        r#"
        SELECT id, Supplier, SupplierNameOriginal, SupplierCountry, VatID, NACE
        FROM suppliers
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows.into_iter().map(|r| r.into()).collect()))
}

pub async fn get_supplier_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<SupplierResponse>, axum::http::StatusCode> {
    // Use `sqlx::query_as!` for compile-time checking
    let row: SupplierDbRow = sqlx::query_as!(
        SupplierDbRow,
        r#"
        SELECT id, Supplier, SupplierNameOriginal, SupplierCountry, VatID, NACE
        FROM suppliers WHERE id = ?1
        "#,
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(row.into()))
}

// --- Transactions ---

pub async fn post_transaction(
    State(state): State<AppState>,
    Json(payload): Json<TransactionLines>,
) -> Result<Json<TransactionResponse>, axum::http::StatusCode> {
    // Bind the dates to variables to ensure they live long enough
    let invoice_date = payload.InvoiceDate.map(|dt| dt.to_rfc3339());
    let due_date = payload.DueDate.map(|dt| dt.to_rfc3339());

    // Use `sqlx::query!` for compile-time checking of the SQL query
    let result = sqlx::query!(
        r#"
        INSERT INTO transactions
        (InvoiceNumber, Supplier, InvoiceDate, DueDate, TransactionValueNOK,
         SpendCategoryL1, SpendCategoryL2, SpendCategoryL3, SpendCategoryL4)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        payload.InvoiceNumber,
        payload.Supplier,
        invoice_date,    // Use the pre-bound variable
        due_date,        // Use the pre-bound variable
        payload.TransactionValueNOK,
        payload.SpendCategoryL1,
        payload.SpendCategoryL2,
        payload.SpendCategoryL3,
        payload.SpendCategoryL4,
    )
    .execute(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.last_insert_rowid();

    let db_row = TransactionDbRow {
        id,
        InvoiceNumber: payload.InvoiceNumber,
        Supplier: payload.Supplier,
        InvoiceDate: invoice_date,    // Use the pre-bound variable
        DueDate: due_date,            // Use the pre-bound variable
        TransactionValueNOK: payload.TransactionValueNOK,
        SpendCategoryL1: payload.SpendCategoryL1,
        SpendCategoryL2: payload.SpendCategoryL2,
        SpendCategoryL3: payload.SpendCategoryL3,
        SpendCategoryL4: payload.SpendCategoryL4,
    };

    Ok(Json(db_row.into_response()))
}

pub async fn get_transactions(
    State(state): State<AppState>,
) -> Result<Json<Vec<TransactionResponse>>, axum::http::StatusCode> {
    // Use `sqlx::query_as!` for compile-time checking
    let rows: Vec<TransactionDbRow> = sqlx::query_as!(
        TransactionDbRow,
        r#"
        SELECT id, Supplier, InvoiceNumber, InvoiceDate, DueDate, TransactionValueNOK, SpendCategoryL1, SpendCategoryL2, SpendCategoryL3, SpendCategoryL4
        FROM transactions
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows.into_iter().map(|r| r.into_response()).collect()))
}

pub async fn get_transaction_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<TransactionResponse>, axum::http::StatusCode> {
    // Use `sqlx::query_as!` for compile-time checking
    let row: TransactionDbRow = sqlx::query_as!(
        TransactionDbRow,
        r#"
        SELECT id, Supplier, InvoiceNumber, InvoiceDate, DueDate, TransactionValueNOK, SpendCategoryL1, SpendCategoryL2, SpendCategoryL3, SpendCategoryL4
        FROM transactions WHERE id = ?1
        "#,
        id,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(row.into_response()))
}

// Get the total amount spent (sum of TransactionValueNOK)
pub async fn get_total_spent(
    State(state): State<AppState>,
) -> Result<Json<f64>, axum::http::StatusCode> {
    // Extract the total_spent value from the Record
    let result = sqlx::query!(
        r#"
        SELECT SUM(TransactionValueNOK) AS total_spent FROM transactions
        "#,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Ensure we extract the `total_spent` field
    Ok(Json(result.total_spent.unwrap_or(0.0))) // Use `unwrap_or` in case `total_spent` is NULL
}

// Get the total number of suppliers
pub async fn get_total_suppliers(
    State(state): State<AppState>,
) -> Result<Json<i64>, axum::http::StatusCode> {
    // Extract the total_suppliers value from the Record
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) AS total_suppliers FROM suppliers
        "#,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Ensure we extract the `total_suppliers` field
    Ok(Json(result.total_suppliers as i64))
}