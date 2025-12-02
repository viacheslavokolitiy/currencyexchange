ALTER TABLE currencies
ADD CONSTRAINT currency_code_unique UNIQUE (currency_code);