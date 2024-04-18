-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE user_role AS ENUM ('admin','creator','customer');

CREATE TABLE IF NOT EXISTS "users"
(
    id                UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name              VARCHAR(100) NOT NULL,
    email             VARCHAR(255) NOT NULL UNIQUE,
    photo             VARCHAR      NOT NULL             DEFAULT 'default.png',
    verified          BOOLEAN      NOT NULL             DEFAULT FALSE,
    password          VARCHAR(100) NOT NULL,
    role              user_role    NOT NULL             DEFAULT 'customer',
    verification_code VARCHAR(100) NOT NULL             DEFAULT '',
    created_at        TIMESTAMP WITH TIME ZONE          DEFAULT NOW(),
    updated_at        TIMESTAMP WITH TIME ZONE          DEFAULT NOW()
);
CREATE INDEX users_email_idx ON users (email);

CREATE TABLE IF NOT EXISTS deals
(
    id              UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title           VARCHAR(255)     NOT NULL UNIQUE,
    description     TEXT             NOT NULL,
    category        TEXT[],
    is_free         BOOLEAN                   DEFAULT NULL,
    price           DOUBLE PRECISION,
    offer_price     DOUBLE PRECISION          DEFAULT NULL,
    expiration_date VARCHAR(100),
    provider        VARCHAR(100),
    provider_url    VARCHAR,
    thumbnail       VARCHAR(255),
    images          TEXT[],
    user_id         VARCHAR                   DEFAULT NULL,
    video_url       VARCHAR                   DEFAULT NULL,
    published       BOOLEAN                   DEFAULT FALSE,
    created_at      TIMESTAMP WITH TIME ZONE  DEFAULT NOW(),
    updated_at      TIMESTAMP WITH TIME ZONE  DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS categories
(
    id         UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title      VARCHAR(255)     NOT NULL,
    thumbnail  VARCHAR(255)              DEFAULT NULL,
    user_id    VARCHAR(255)              DEFAULT NULL,
    status     VARCHAR(155)              DEFAULT NULL,
    main_id    VARCHAR(255)              DEFAULT NULL,
    published  BOOLEAN          NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE  DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE  DEFAULT NOW()
)
