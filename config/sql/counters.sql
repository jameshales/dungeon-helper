CREATE TABLE counters (
  channel_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  counter INTEGER NOT NULL,
  PRIMARY KEY (channel_id, user_id)
);
