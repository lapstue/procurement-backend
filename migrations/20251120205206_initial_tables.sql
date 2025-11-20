CREATE TABLE IF NOT EXISTS suppliers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    Supplier TEXT NOT NULL,
    SupplierNameOriginal TEXT NOT NULL,
    SupplierCountry TEXT NOT NULL,
    VatID TEXT NOT NULL,
    NACE TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    Supplier TEXT NOT NULL,
    InvoiceNumber TEXT NOT NULL,
    InvoiceDate TEXT,
    DueDate TEXT,
    TransactionValueNOK REAL NOT NULL,
    SpendCategoryL1 TEXT NOT NULL,
    SpendCategoryL2 TEXT NOT NULL,
    SpendCategoryL3 TEXT NOT NULL,
    SpendCategoryL4 TEXT NOT NULL
);