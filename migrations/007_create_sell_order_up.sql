CREATE TABLE IF NOT EXISTS sell_orders(
    id SERIAL PRIMARY KEY,
    issuer_id INT NOT NULL,
    sell_volume INT NOT NULL,
    sell_currency_code VARCHAR(255),
    buy_currency_code VARCHAR(255),
    buy_sell_exchange_ratio FLOAT(24),
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    FOREIGN KEY (issuer_id) REFERENCES users(id)
)