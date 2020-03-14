CREATE TABLE messages (
  message_id TEXT PRIMARY KEY,
  channel_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  content TEXT NOT NULL,
  corrected_content TEXT NULL,
  posted TIMESTAMP NOT NULL,
  intent_name TEXT NULL,
  confidence_score REAL NOT NULL
);

CREATE TABLE slots (
  message_id TEXT NOT NULL REFERENCES messages (message_id),
  slot_index TEXT NOT NULL,
  raw_value TEXT NOT NULL,
  value TEXT NULL,
  slot_name TEXT NOT NULL,
  confidence_score REAL NULL,
  PRIMARY KEY (message_id, slot_index)
);
