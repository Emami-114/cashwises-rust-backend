-- Add up migration script here

CREATE TABLE IF NOT EXISTS tags (
id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
title VARCHAR(255) NOT NULL UNIQUE,
created_at        TIMESTAMP WITH TIME ZONE          DEFAULT NOW()
)