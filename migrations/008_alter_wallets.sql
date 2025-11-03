ALTER TABLE wallets
DROP CONSTRAINT wallets_user_id_key,
ALTER COLUMN user_id SET NOT NULL;