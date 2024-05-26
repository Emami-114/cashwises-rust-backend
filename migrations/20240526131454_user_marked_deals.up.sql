-- Add up migration script here

CREATE TABLE IF NOT EXISTS user_marked_deals (
user_id UUID NOT NULL REFERENCES users(id),
deal_id UUID NOT NULL REFERENCES deals(id),
PRIMARY KEY (user_id,deal_id)
)