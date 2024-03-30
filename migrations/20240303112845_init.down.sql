-- Add down migration script here


DROP TABLE IF EXISTS deals;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS categories;
DROP EXTENSION IF EXISTS "uuid-ossp";
DROP TYPE IF EXISTS user_role;