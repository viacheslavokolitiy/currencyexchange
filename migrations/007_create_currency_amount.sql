CREATE TABLE IF NOT EXISTS currency_amount(
    id SERIAL PRIMARY KEY,
    amount INTEGER,
    currency_id INTEGER UNIQUE NOT NULL,
    FOREIGN KEY (currency_id) REFERENCES currencies(currency_id) ON DELETE CASCADE ON UPDATE CASCADE
)