-- Add migration script here
ALTER TABLE users 
ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NULL;