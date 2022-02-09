-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   username TEXT NOT NULL,
   first_name TEXT NOT NULL,
   last_name TEXT NOT NULL,
   email TEXT UNIQUE NOT NULL,
   mobile TEXT NOT NULL
);