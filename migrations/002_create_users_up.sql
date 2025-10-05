CREATE TABLE IF NOT EXISTS users(
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(12) UNIQUE NOT NULL,
    email VARCHAR(120) UNIQUE NOT NULL,
    password VARCHAR(25) NOT NULL,
    firstname VARCHAR(25) NOT NULL,
    middlename VARCHAR(20) NULL,
    lastname VARCHAR(35) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
)