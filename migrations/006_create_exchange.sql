CREATE TABLE IF NOT EXISTS currency_exchange(
    id SERIAL PRIMARY KEY,
    income FLOAT,
    sell_order_id INTEGER NOT NULL,
    buy_order_id INTEGER NOT NULL,
    FOREIGN KEY (sell_order_id) REFERENCES sell_orders(sell_order_id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (buy_order_id) REFERENCES buy_orders(buy_order_id) ON DELETE CASCADE ON UPDATE CASCADE
)