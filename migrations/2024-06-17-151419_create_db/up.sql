-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  account TEXT NOT NULL,
  password TEXT NOT NULL,
  identity Boolean NOT NULL
);

CREATE TABLE IF NOT EXISTS website_account (
  id INTEGER PRIMARY KEY,
  account TEXT NOT NULL,
  password TEXT NOT NULL,
  site_name TEXT NOT NULL,
  site_url TEXT NOT NULL,
  note TEXT
);

CREATE TABLE IF NOT EXISTS users_account(
  account_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  PRIMARY KEY (account_id, user_id),
  FOREIGN KEY (account_id) REFERENCES website_account(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);
