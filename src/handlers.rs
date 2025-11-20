// --- Suppliers ---

use axum::{
    Json,
    extract::{Path, State}, response::Html,
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

pub async fn serve_dashboard() -> Html<&'static str> {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Procurement Dashboard</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    padding: 20px;
                    background-color: #f4f4f9;
                }
                .header-box {
                    display: inline-block;
                    width: 48%;
                    padding: 20px;
                    margin: 10px;
                    background-color: #2d2d2d;
                    color: white;
                    text-align: center;
                    border-radius: 8px;
                }
                .header-box h3 {
                    margin: 0;
                    font-size: 24px;
                }
                .header-box p {
                    font-size: 30px;
                    margin: 10px 0 0;
                }
                .transaction-table {
                    width: 100%;
                    border-collapse: collapse;
                    margin-top: 30px;
                }
                .transaction-table th, .transaction-table td {
                    border: 1px solid #ddd;
                    padding: 12px;
                    text-align: left;
                }
                .transaction-table th {
                    background-color: #f4f4f9;
                }
            </style>
        </head>
        <body>
            <h1>Procurement Dashboard</h1>
            
            <div class="header-box" id="total-spent">
                <h3>Total Spent (NOK)</h3>
                <p>Loading...</p>
            </div>

            <div class="header-box" id="total-suppliers">
                <h3>Total Suppliers</h3>
                <p>Loading...</p>
            </div>

            <h2>Transactions</h2>
            <table class="transaction-table" id="transaction-table">
                <thead>
                    <tr>
                        <th>Invoice Number</th>
                        <th>Supplier</th>
                        <th>Invoice Date</th>
                        <th>Transaction Value (NOK)</th>
                        <th>Spend Category L1</th>
                        <th>Spend Category L2</th>
                    </tr>
                </thead>
                <tbody>
                    <!-- Transaction rows will be populated here -->
                </tbody>
            </table>

            <script>
                // Function to fetch data and update the UI
                async function fetchData() {
                    try {
                        const totalSpentRes = await fetch('/transactions/total_spent');
                        const totalSpent = await totalSpentRes.json();
                        document.getElementById('total-spent').querySelector('p').textContent = totalSpent;

                        const totalSuppliersRes = await fetch('/suppliers/total_suppliers');
                        const totalSuppliers = await totalSuppliersRes.json();
                        document.getElementById('total-suppliers').querySelector('p').textContent = totalSuppliers;

                        const transactionsRes = await fetch('/transactions');
                        const transactions = await transactionsRes.json();

                        const tableBody = document.getElementById('transaction-table').querySelector('tbody');
                        tableBody.innerHTML = ''; // Clear any existing rows
                        transactions.forEach(transaction => {
                            const row = document.createElement('tr');
                            row.innerHTML = `
                                <td>${transaction.InvoiceNumber}</td>
                                <td>${transaction.Supplier}</td>
                                <td>${transaction.InvoiceDate}</td>
                                <td>${transaction.TransactionValueNOK}</td>
                                <td>${transaction.SpendCategoryL1}</td>
                                <td>${transaction.SpendCategoryL2}</td>
                            `;
                            tableBody.appendChild(row);
                        });
                    } catch (error) {
                        console.error("Error fetching data:", error);
                    }
                }

                // Fetch data when the page loads
                window.onload = fetchData;
            </script>
        </body>
        </html>
    "#;

    Html(html) // Return the HTML content properly as an HTML response
}