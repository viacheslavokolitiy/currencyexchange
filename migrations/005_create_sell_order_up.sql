CREATE TABLE IF NOT EXISTS sell_orders(
    sell_order_id SERIAL PRIMARY KEY,
    issuer_id INTEGER UNIQUE NOT NULL,
    sell_currency_amount INTEGER NOT NULL,
    exchange_rate FLOAT NOT NULL,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    sell_currency_id INTEGER UNIQUE NOT NULL,
    buy_currency_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY(issuer_id) REFERENCES users(user_id),
    FOREIGN KEY(buy_currency_id) REFERENCES currencies(currency_id),
    FOREIGN KEY(sell_currency_id) REFERENCES currencies(currency_id)
)