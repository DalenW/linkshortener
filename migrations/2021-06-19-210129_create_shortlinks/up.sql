-- Your SQL goes here
CREATE TABLE shortlinks (
  id TEXT PRIMARY KEY,
  link TEXT NOT NULL,
  created_at TIMESTAMP,
  enabled BOOLEAN NOT NULL DEFAULT 't',
  hits BIGINT DEFAULT 0
)