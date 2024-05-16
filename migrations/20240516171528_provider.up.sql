-- Add up migration script here

CREATE TABLE IF NOT EXISTS providers (
id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
title VARCHAR(255) NOT NULL UNIQUE,
logo VARCHAR,
url VARCHAR
);