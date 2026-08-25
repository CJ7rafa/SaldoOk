PRAGMA foreign_keys = ON;

-- 1. Catalog of Spenders / Persons (e.g., 'Rafa', 'Papá', 'Negocio')
CREATE TABLE IF NOT EXISTS spenders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    active INTEGER NOT NULL DEFAULT 1 -- 1: Active, 0: Inactive (Soft delete)
);

-- 2. Catalog of Expense Categories (e.g., 'Ferretería', 'Transporte', 'Almuerzo')
CREATE TABLE IF NOT EXISTS expense_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT 'BLUE' CHECK (color IN ('RED', 'BLUE', 'GREEN', 'BLACK', 'ORANGE')),
    active INTEGER NOT NULL DEFAULT 1
);

-- 3. Main Expenses Table (Cash Outflow Transactions)
CREATE TABLE IF NOT EXISTS expenses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    spender_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    amount REAL NOT NULL CHECK (amount > 0),
    expense_date TEXT NOT NULL, -- ISO-8601 Format: 'YYYY-MM-DD'
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    
    FOREIGN KEY (spender_id) REFERENCES spenders(id) 
        ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (category_id) REFERENCES expense_categories(id) 
        ON UPDATE CASCADE ON DELETE RESTRICT
);

-- 4. Indexes for fast reporting and queries on Celeron N4000
CREATE INDEX IF NOT EXISTS idx_expenses_date ON expenses(expense_date);
CREATE INDEX IF NOT EXISTS idx_expenses_spender ON expenses(spender_id);
CREATE INDEX IF NOT EXISTS idx_expenses_category ON expenses(category_id);