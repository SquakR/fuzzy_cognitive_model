-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    is_email_confirmed BOOLEAN NOT NULL DEFAULT FAlSE,
    first_name VARCHAR(255) NOT NULL,
    second_name VARCHAR(255) NULL,
    last_name VARCHAR(255) NOT NULL,
    avatar VARCHAR,
    language VARCHAR(5),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TABLE email_confirmations (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    email VARCHAR(255) NOT NULL,
    is_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('email_confirmations')