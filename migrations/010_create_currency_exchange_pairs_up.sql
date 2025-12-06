CREATE TABLE IF NOT EXISTS currency_exchange_ratios(
    id SERIAL PRIMARY KEY,
    first_currency_code VARCHAR(255),
    second_currency_code VARCHAR(255),
    exchange_ratio FLOAT(24)
)