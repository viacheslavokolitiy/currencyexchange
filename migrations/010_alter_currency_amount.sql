ALTER TABLE currency_amount
    ADD COLUMN wallet_id INT,
  ADD CONSTRAINT fk_wallet_id
    FOREIGN KEY (wallet_id)
    REFERENCES wallets (wallet_id)
    ON DELETE CASCADE
    ON UPDATE CASCADE;