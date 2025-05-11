-- Your SQL goes here
CREATE TABLE users (
    id          BIGINT PRIMARY KEY,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL,
    modified_at TIMESTAMP WITH TIME ZONE NULL,
    name        VARCHAR(100) NOT NULL,
    email       VARCHAR NOT NULL
)