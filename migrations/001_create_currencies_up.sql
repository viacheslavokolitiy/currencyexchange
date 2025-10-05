CREATE TABLE IF NOT EXIST currencies(
    currency_id SERIAL PRIMARY KEY,
    currency_code VARCHAR(10)
)