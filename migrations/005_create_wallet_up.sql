CREATE TABLE IF NOT EXISTS wallets(
    id SERIAL PRIMARY KEY,
    currency_amount FLOAT(24),
    currency_code VARCHAR(255),
    user_id INT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);