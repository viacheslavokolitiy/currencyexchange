CREATE TABLE IF NOT EXISTS buy_orders(
    id SERIAL PRIMARY KEY,
    issuer_id INT NOT NULL,
    buy_volume INT NOT NULL,
    buy_currency_code VARCHAR(255),
    sell_currency_code VARCHAR(255),
    buy_sell_exchange_ratio FLOAT(24),
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    FOREIGN KEY (issuer_id) REFERENCES users(id)
)