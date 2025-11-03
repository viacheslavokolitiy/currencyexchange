ALTER TABLE users
ALTER COLUMN username TYPE VARCHAR(120),
    ALTER COLUMN username SET NOT NULL,
    ADD CONSTRAINT username_unique UNIQUE (username);