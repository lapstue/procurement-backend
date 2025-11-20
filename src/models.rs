use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Deserialize, Serialize, Debug)]
pub struct SupplierLines {
    pub Supplier: String,
    pub SupplierNameOriginal: String,
    pub SupplierCountry: String,
    pub VatID: String,
    pub NACE: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct SupplierDbRow {
    pub id: i64,
    pub Supplier: String,
    pub SupplierNameOriginal: String,
    pub SupplierCountry: String,
    pub VatID: String,
    pub NACE: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SupplierResponse {
    pub id: i64,
    pub Supplier: String,
    pub SupplierNameOriginal: String,
    pub SupplierCountry: String,
    pub VatID: String,
    pub NACE: String,
}

impl From<SupplierDbRow> for SupplierResponse {
    fn from(row: SupplierDbRow) -> Self {
        SupplierResponse {
            id: row.id,
            Supplier: row.Supplier,
            SupplierNameOriginal: row.SupplierNameOriginal,
            SupplierCountry: row.SupplierCountry,
            VatID: row.VatID,
            NACE: row.NACE,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionLines {
    pub InvoiceNumber: String,
    pub Supplier : String,
    pub InvoiceDate: Option<DateTime<Utc>>,
    pub DueDate: Option<DateTime<Utc>>,
    pub TransactionValueNOK: f64,
    pub SpendCategoryL1: String,
    pub SpendCategoryL2: String,
    pub SpendCategoryL3: String,
    pub SpendCategoryL4: String,
}

#[derive(sqlx::FromRow)]
pub struct TransactionDbRow {
    pub id: i64,
    pub InvoiceNumber: String,
    pub Supplier : String,
    pub InvoiceDate: Option<String>,
    pub DueDate: Option<String>,
    pub TransactionValueNOK: f64,
    pub SpendCategoryL1: String,
    pub SpendCategoryL2: String,
    pub SpendCategoryL3: String,
    pub SpendCategoryL4: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionResponse {
    pub id: i64,
    pub InvoiceNumber: String,
    pub Supplier : String,
    pub InvoiceDate: Option<DateTime<Utc>>,
    pub DueDate: Option<DateTime<Utc>>,
    pub TransactionValueNOK: f64,
    pub SpendCategoryL1: String,
    pub SpendCategoryL2: String,
    pub SpendCategoryL3: String,
    pub SpendCategoryL4: String,
}

impl TransactionDbRow {
    pub fn into_response(self) -> TransactionResponse {
        TransactionResponse {
            id: self.id,
            InvoiceNumber: self.InvoiceNumber,
            Supplier : self.Supplier,
            InvoiceDate: self
                .InvoiceDate
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            DueDate: self
                .DueDate
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            TransactionValueNOK: self.TransactionValueNOK,
            SpendCategoryL1: self.SpendCategoryL1,
            SpendCategoryL2: self.SpendCategoryL2,
            SpendCategoryL3: self.SpendCategoryL3,
            SpendCategoryL4: self.SpendCategoryL4,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
}
