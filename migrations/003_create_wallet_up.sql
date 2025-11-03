CREATE TABLE IF NOT EXISTS wallets(
    wallet_id SERIAL PRIMARY KEY,
    user_id INTEGER UNIQUE NOT NULL,
    currency_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (currency_id) REFERENCES currencies(currency_id) ON DELETE CASCADE ON UPDATE CASCADE
)