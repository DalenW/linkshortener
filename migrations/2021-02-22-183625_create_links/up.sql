-- Your SQL goes here

CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  shorttext VARCHAR NOT NULL,
  hyperlink TEXT NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 't'
)