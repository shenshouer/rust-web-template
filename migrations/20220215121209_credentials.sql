CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS credentials (
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   username VARCHAR(50) NOT NULL unique,
   password TEXT NOT NULL
);