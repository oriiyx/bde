CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(255) NOT NULL UNIQUE,
    email      VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP    NOT NULL DEFAULT NOW()
);

ALTER TABLE users
    ADD COLUMN name VARCHAR(255) NULL;

CREATE TABLE oauth
(
    id               SERIAL PRIMARY KEY,
    user_id          INT REFERENCES users (id) ON DELETE CASCADE,
    provider         VARCHAR(255) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    created_at       TIMESTAMP    NOT NULL DEFAULT NOW()
);