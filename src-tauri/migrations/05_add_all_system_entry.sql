PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS daily_cash_registers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_date TEXT NOT NULL UNIQUE,          -- Fecha del día (YYYY-MM-DD), solo un cierre por día
    
    -- 1. MERCADO
    market_sales REAL NOT NULL DEFAULT 0.0 CHECK (market_sales >= 0),        -- Ventas mercado brutas
    market_vault_saved REAL NOT NULL DEFAULT 0.0 CHECK (market_vault_saved >= 0), -- Dinero que retira para guardar en cajafuerte
    
    -- 2. CASA
    house_sales REAL NOT NULL DEFAULT 0.0 CHECK (house_sales >= 0),          -- Ventas en casa / tienda casa
    
    -- 3. OTROS INGRESOS MERCADO
    chain_income REAL NOT NULL DEFAULT 0.0 CHECK (chain_income >= 0),        -- Cobro de 'cadenas' recibidas
    vault_loan REAL NOT NULL DEFAULT 0.0 CHECK (vault_loan >= 0),            -- Autopréstamo extraído de la cajafuerte
    
    -- 4. RETIROS / AUTOCONSUMO
    self_consumption REAL NOT NULL DEFAULT 0.0 CHECK (self_consumption >= 0),-- Mercadería o dinero tomado para consumo de casa
    
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE INDEX IF NOT EXISTS idx_daily_cash_date ON daily_cash_registers(entry_date);