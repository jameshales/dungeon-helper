CREATE TABLE channels(
  channel_id TEXT PRIMARY KEY,
  enabled BOOLEAN NOT NULL DEFAULT false,
  locked BOOLEAN NOT NULL DEFAULT false,
  dice_only BOOLEAN NOT NULL DEFAULT false
);
