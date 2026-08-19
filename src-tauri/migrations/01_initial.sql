PRAGMA foreign_keys = ON;

-- 1. Suppliers Catalog
CREATE TABLE IF NOT EXISTS suppliers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    active INTEGER NOT NULL DEFAULT 1 -- 1: Active, 0: Inactive (Soft delete)
);

-- 2. Purchases / Invoices
CREATE TABLE IF NOT EXISTS purchases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    supplier_id INTEGER NOT NULL,
    invoice_number TEXT,
    issue_date TEXT NOT NULL,            -- Format 'YYYY-MM-DD'
    due_date TEXT,                       -- Due date if on credit
    total_amount REAL NOT NULL CHECK(total_amount > 0),
    payment_type TEXT NOT NULL CHECK(payment_type IN ('CASH', 'CREDIT')),
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK(status IN ('PENDING', 'PAID', 'VOID')),
    notes TEXT,
    FOREIGN KEY (supplier_id) REFERENCES suppliers(id) ON UPDATE CASCADE ON DELETE RESTRICT
);

-- 3. Payments (Outgoing Cash Flow)
CREATE TABLE IF NOT EXISTS purchase_payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    purchase_id INTEGER NOT NULL,
    payment_date TEXT NOT NULL,          -- Exact date money was paid
    amount REAL NOT NULL CHECK(amount > 0),
    payment_method TEXT NOT NULL DEFAULT 'CASH', -- 'CASH', 'TRANSFER', etc.
    note TEXT,
    FOREIGN KEY (purchase_id) REFERENCES purchases(id) ON UPDATE CASCADE ON DELETE CASCADE
);

-- 4. Indexes for faster reporting and querying
CREATE INDEX IF NOT EXISTS idx_purchases_supplier ON purchases(supplier_id);
CREATE INDEX IF NOT EXISTS idx_purchases_issue_date ON purchases(issue_date);
CREATE INDEX IF NOT EXISTS idx_purchases_status ON purchases(status);
CREATE INDEX IF NOT EXISTS idx_purchase_payments_date ON purchase_payments(payment_date);
