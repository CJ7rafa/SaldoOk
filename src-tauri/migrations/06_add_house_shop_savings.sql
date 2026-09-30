-- Agregando columna F/E Ahorros (Tienda Casa)
ALTER TABLE daily_cash_registers 
ADD COLUMN house_shop_savings REAL NOT NULL DEFAULT 0.0 CHECK (house_shop_savings >= 0);
