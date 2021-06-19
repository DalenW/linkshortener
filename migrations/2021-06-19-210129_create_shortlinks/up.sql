-- Your SQL goes here
CREATE TABLE shortlinks (
  id TEXT PRIMARY KEY,
  link TEXT NOT NULL,
  created_at timestamp with TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  enabled BOOLEAN NOT NULL DEFAULT 't',
  hits BIGINT NOT NULL DEFAULT 0
)