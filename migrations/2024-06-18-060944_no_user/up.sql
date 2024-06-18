-- Your SQL goes here
DROP TABLE IF EXISTS users_account;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS website_account;

CREATE TABLE IF NOT EXISTS website_account (
  id INTEGER PRIMARY KEY,
  account TEXT NOT NULL,
  password TEXT NOT NULL,
  site_url TEXT NOT NULL,
  site_name TEXT,
  note TEXT
);
