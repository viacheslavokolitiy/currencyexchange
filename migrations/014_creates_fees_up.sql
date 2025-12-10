CREATE TABLE IF NOT EXISTS fees(
    id SERIAL PRIMARY KEY,
    exchange_comission FLOAT(24),
    state_tax FLOAT(24),
    sales_tax FLOAT(24)
)